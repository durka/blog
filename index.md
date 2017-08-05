---
layout: default
---
## Blog goes here

### Posts
{% for post in site.posts %}
- [{{ post.title }}]({{ post.url }}) ({{ post.date | date: "%b %d %Y" }})
{% endfor %}

