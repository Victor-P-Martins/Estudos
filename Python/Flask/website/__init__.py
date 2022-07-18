from flask import Flask
from traitlets import import_item

def create_app():
    app  = Flask(__name__)
    #Criando uma key de encriptação dos cookies na sessão. Pode ser qualquer string
    app.config['SECRET_KEY'] = 'asidunPOIENOASIND   OJNASDAS    '

    from .views import views
    from .auth import auth
    app.register_blueprint(views,url_prefix = '/')
    app.register_blueprint(auth,url_prefix = '/')

    return app