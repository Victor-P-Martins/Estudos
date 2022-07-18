from website import create_app

#Inicializando o app
app = create_app()

#debug = True faz com que sempre que houver uma mudança no arquivo haverá o refresh no webserver
if __name__ == '__main__':
    app.run(debug = True)