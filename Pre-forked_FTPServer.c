#include <sys/socket.h>
#include <arpa/inet.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/sendfile.h>
#include <fcntl.h>
#include <getopt.h>
#include <limits.h>
#include <unistd.h>
#include <pthread.h>

int conexiones, puerto;
char ruta[1024];
struct sockaddr_in server, client;
struct stat obj;
int sock2;
char buf[100], command[5], filename[20];
int k, i, size, len, c;
int filehandle;
pid_t * procesos;

void* procesar_solicitud(int* cliente);

struct nodo{
    struct nodo* sig;
    int *cliente_socket;
};
typedef struct nodo nodo_t;

nodo_t* cabeza = NULL;
nodo_t* cola = NULL;

void encolar(int* cliente_socket){
    nodo_t *nuevonodo = malloc(sizeof(nodo_t));
    nuevonodo->cliente_socket = cliente_socket;
    nuevonodo->sig = NULL;
    if(cola == NULL){
        cabeza = nuevonodo;
    }
    else{
        cola->sig = nuevonodo;
    }
    cola = nuevonodo;

}

int* desencolar(){
    if(cabeza == NULL){
        return NULL;
    }
    else{
        int* resultado = cabeza->cliente_socket;
        nodo_t* temporal = cabeza;
        cabeza = cabeza->sig;
        if(cabeza == NULL){
            cola = NULL;
        }
        free(temporal);
        return resultado;
    }
}

void* procesar(void*arg){
    
    while (true){

        int* cliente ;

        cliente = desencolar();

        if(cliente != NULL){
            procesar_solicitud(cliente);
        }
    }
    pthread_exit(NULL);
}

void crear_procesos(){
	 for(int i = 0; i<conexiones;i++ ){
        procesos[i] = fork();
        pause();  
        
    }
}

void* procesar_solicitud(int* cliente){
  i = 0;
  while(recv(*cliente, buf, 100, 0)){
      
      sscanf(buf, "%s", command);
      printf("%s", command);
      
      if(!strcmp(command, "ls")){

	      system("ls >temps.txt");
	      i = 0;
	      stat("temps.txt",&obj);
	      size = obj.st_size;
	      send(sock2, &size, sizeof(int),0);
	      filehandle = open("temps.txt", O_RDONLY);
	      sendfile(sock2,filehandle,NULL,size);
	    }
      else if(!strcmp(command,"get")){

	      sscanf(buf, "%s%s", filename, filename);
	      stat(filename, &obj);
	      filehandle = open(filename, O_RDONLY);
	      size = obj.st_size;
	      if(filehandle == -1)
	        size = 0;
	      send(sock2, &size, sizeof(int), 0);
	      if(size)
	        sendfile(sock2, filehandle, NULL, size);
      
	    }
      else if(!strcmp(command, "put"))
        {
	  int c = 0, len;
	  char *f;
	  sscanf(buf+strlen(command), "%s", filename);
	  recv(sock2, &size, sizeof(int), 0);
	  i = 1;

	  while(1)
	    {
	      filehandle = open(filename, O_CREAT | O_EXCL | O_WRONLY, 0666);
	      if(filehandle == -1)
		{
		  sprintf(filename + strlen(filename), "%d", i);
		}
	      else
		break;
	    }
	  f = malloc(size);
	  recv(sock2, f, size, 0);
	  c = write(filehandle, f, size);
	  close(filehandle);
	  send(sock2, &c, sizeof(int), 0);
        }
      else if(!strcmp(command, "connect"))
	{
	  system("pwd>temp.txt");
	  i = 0;
          FILE*f = fopen("temp.txt","r");
          while(!feof(f))
            buf[i++] = fgetc(f);
          buf[i-1] = '\0';
	  fclose(f);
          send(sock2, buf, 100, 0);
	}
      else if(!strcmp(command, "cd"))
        {
          if(chdir(buf+3) == 0)
	    c = 1;
	  else
	    c = 0;
          send(sock2, &c, sizeof(int), 0);
        }


      else if(!strcmp(command, "bye") || !strcmp(command, "disconnect"))
	{
	  i = 1;
	  send(sock2, &i, sizeof(int), 0);
	  close(sock2);
    break;
	}
    }

}

int main(int argc,char *argv[])
{
  printf("debbug\n");
  int opcion;
  

	while((opcion = getopt(argc,argv,"n:w:p:")) != -1)
	{
		switch (opcion)
		{
			case 'n'://numero de conexiones maximas
				conexiones = atoi(optarg);

			case 'w':// direccion root del servidor
				strcpy(ruta, optarg); 
				
			case 'p':// puerto del servidor
				puerto = atoi(optarg);
            
			}
			
	}

printf("Conexiones: %d\n", conexiones);

  
  int sock1;
  sock1 = socket(AF_INET, SOCK_STREAM, 0);
  printf("socket: %d\n", sock1);
  if(sock1 == -1)
    {
      printf("Socket creation failed\n");
      exit(1);
    }
  
  server.sin_port = puerto;
  server.sin_addr.s_addr = 0;
  server.sin_family = AF_INET;
  k = bind(sock1,(struct sockaddr*)&server,sizeof(server));
  if(k == -1)
    {
      printf("Binding error\n");
      exit(1);
    }
   
  k = listen(sock1,1);
  if(k == -1)
    {
      printf("Listen failed");
      exit(1);
    }

  pid_t proc_aux[conexiones];
  procesos = &proc_aux[0];
  crear_hilos();
  chdir(ruta);


  while(true){
    len = sizeof(client);
    sock2 = accept(sock1,(struct sockaddr*)&client, &len);

    if(conexiones > 0){
        conexiones--;
    
    }else{
        send(sock2, buf, 100, 0);
        close(sock2);
    }

        int* cliente = malloc(sizeof(int));
        *cliente = sock2;

        encolar(cliente);


  }

  return 0;

}