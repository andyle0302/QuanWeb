from .models import User
from .views import authm

from quanweb.common import loginmanager

@loginmanager.user_loader
def load_user(uid):
    return User.query.get(uid)
