# socket-server

## Execução local

### Compilando o projeto e criando a imagem
```
docker build -t server-socket .
```

### Executando o container
```
docker run --rm --net=host -e PORT=$ANY_PORT_YOU_WANT -p $ANY_PORT_YOU_WANT:$ANY_PORT_YOU_WANT -t server-socket
```