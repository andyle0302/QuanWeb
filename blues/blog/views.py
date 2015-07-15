from jinja2 import TemplateNotFound
from sqlalchemy.orm import load_only
from flask import Blueprint
from sqlalchemy.orm.exc import NoResultFound
from flask import request, render_template, abort, redirect, url_for

from quanweb import config
from quanweb.common import UNCATEGORIZED
from .models import db, Entry, Category

blogm = Blueprint('blog', __name__, static_folder=config.STATIC_FOLDER,
                  template_folder=config.TEMPLATE_FOLDER)


@blogm.route('/<int:year>/<int:month>/<int:pk>/<slug>')
def show_post(year, month, pk, slug):
    entry = Entry.query.get(pk)
    siblings = Entry.pub().options(load_only('id', 'date_published'))
    cat = request.args.get('cat')
    if cat:
        siblings = siblings.join(Entry.categories).filter(Category.slug == cat)
    next_entry = siblings.filter(Entry.id>pk).first()
    prev_entry = siblings.filter(Entry.id<pk).order_by(Entry.id.desc()).first()
    return render_template('blog/entry.html', entry=entry,
                           prev_entry=prev_entry,
                           next_entry=next_entry,
                           catslug=cat)


@blogm.route('/<int:year>/<int:month>/<int:pk>')
def show_post_short(year, month, pk):
    ''' Redirect to correct, full URL '''
    try:
        entry = Entry.query.get(pk)
    except NoResultFound:
        abort(404)
    date_published = entry.date_published
    year, month = date_published.year, date_published.month
    full_url = url_for('blog.show_post',
                       year=year, month=month, pk=pk, slug=entry.slug)
    return redirect(full_url, 301)


@blogm.route('/<catslug>')
@blogm.route('/')
def list_posts(catslug=None):
    cvars = {}
    query = Entry.pub().order_by(Entry.date_published.desc())
    if catslug == UNCATEGORIZED:
        entries = query.filter_by(categories=None)
    elif catslug:
        category = Category.query.filter_by(slug=catslug).one()
        cvars['cat'] = category
        entries = category.entries.all()
    else:
        entries = query.all()
    cvars['entries'] = entries
    return render_template('blog/entries.html', **cvars)
