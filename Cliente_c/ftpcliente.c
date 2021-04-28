#include <sys/socket.h>
#include <netinet/in.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include<sys/stat.h>
#include<sys/sendfile.h>
#include<fcntl.h>
"Este archivo es un cliente FTP el cual permite descargar un binario a través de una lista de comandos en los parámetros o bien interactuar con el servidor FTP como cualquier otro cliente FTP."
int main(int argc,char *argv[])
{

  "Seteo de las variables necesarias para la creación del cliente"
  struct sockaddr_in server;
  struct stat obj;
  int sock;
  int choice;
  char buf[100], command[5], filename[20], *f;
  int k, size, status;
  int filehandle;
  sock = socket(AF_INET, SOCK_STREAM, 0);   //creación del sock


  if(sock == -1)
    {
      printf("Falló en creación del Socket");   //validación del sock
      exit(1);
    }
  server.sin_family = AF_INET;
  server.sin_port = atoi(argv[1]);
  server.sin_addr.s_addr = 0;
  k = connect(sock,(struct sockaddr*)&server, sizeof(server));   //conexión al servidor
  if(k == -1)
    {
      printf("Error de conexión.");
      exit(1);
    }
  int i = 1;
  "Este ciclo es para recorrer la línea de comandos a ejecutar"
  for(int x = 2; i<argc;x++){
      
	if(strcmp(argv[x] ,"get") == 0)     //caso del comando get
	{
	  printf("Nombre de archivo a descargar: ");
	  scanf("%s", filename);
	  strcpy(buf, "get ");
	  strcat(buf, filename);
	  send(sock, buf, 100, 0);
	  recv(sock, &size, sizeof(int), 0);
	  if(!size)
	    {
	      printf("EL archivo no existe.\n\n");

	    }
	  f = malloc(size);
	  recv(sock, f, size, 0);
	  while(1)
	    {
	      filehandle = open(filename, O_CREAT | O_EXCL | O_WRONLY, 0666);
	      if(filehandle == -1)
		{
		  sprintf(filename + strlen(filename), "%d", i);
		}
	      else break;
		}
	  write(filehandle, f, size, 0);
	  close(filehandle);
	  strcpy(buf, "cat ");
	  strcat(buf, filename);
	  system(buf);

	}
	else if(strcmp( argv[x] ,"put") == 0) //caso del comando put
	{
	  printf("Nombre de archivo a cargar: ");
      scanf("%s", filename);
	  filehandle = open(filename, O_RDONLY);
      if(filehandle == -1){
      	printf("El archivo no existe.\n\n");

    	}
      strcpy(buf, "put ");
	  strcat(buf, filename);
	  send(sock, buf, 100, 0);
	  stat(filename, &obj);
	  size = obj.st_size;
	  send(sock, &size, sizeof(int), 0);
	  sendfile(sock, filehandle, NULL, size);
	  recv(sock, &status, sizeof(int), 0);
	  if(status)
	    printf("Archivo subido exitosamente.\n");
	  else
	    printf("Fallo al enviar el archivo.\n");

	}
	else if(strcmp( argv[x] ,"connect") == 0)  //caso del comando connect
	{
	  strcpy(buf, "connect");
	  send(sock, buf, 100, 0);
	  recv(sock, buf, 100, 0);
	  printf("El servider dice hola desde: %s\n", buf);

	}
	else if(strcmp( argv[x] ,"ls") == 0)  //caso del comando ls
	{
	  strcpy(buf, "ls");
          send(sock, buf, 100, 0);
	  recv(sock, &size, sizeof(int), 0);
          f = malloc(size);
          recv(sock, f, size, 0);
	  filehandle = creat("temp.txt", O_WRONLY);
	  write(filehandle, f, size, 0);
	  close(filehandle);
          printf("Archivos en servidor remoto:\n");
	  system("sudo cat temp.txt");
	  system("unlink temp.txt");

	}
	else if(strcmp( argv[x] ,"cd") == 0)  //caso del comando cd
	{
	  strcpy(buf, "cd ");
	  printf("Ingrese el directorio: ");
	  scanf("%s", buf + 3);
          send(sock, buf, 100, 0);
	  recv(sock, &status, sizeof(int), 0);
          if(status)
            printf("Movido al nuevo directorio.\n");
          else
            printf("Fallo al cambiar de directorio.\n");
         
	}
	else if(strcmp( argv[x] ,"disconnect") == 0)  //caso del comando disconnect
	{
	  strcpy(buf, "disconnect");
          send(sock, buf, 100, 0);
          recv(sock, &status, 100, 0);
	  if(status)
	    {
	      printf("Desconectando del servidor.\n");
	      exit(0);
	    }
	}
    }

}
