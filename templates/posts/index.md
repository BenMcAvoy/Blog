# All posts

{% for post in posts %}
<h2>{{ post.name }}</h2>
<a href="{{ post.url | safe }}">Read more</a>
{% endfor %}
