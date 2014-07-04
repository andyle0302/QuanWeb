from flask import Blueprint, render_template, abort
from jinja2 import TemplateNotFound

from quanweb import config
from blog.models import Entry

frontpage = Blueprint('frontpage', __name__, static_folder=config.STATIC_FOLDER,
                       template_folder=config.TEMPLATE_FOLDER)

@frontpage.route('/')
def index():
    posts = Entry.pub().order_by(Entry.date_published.desc()).limit(5).all()
    return render_template('front/index.html', posts=posts)
