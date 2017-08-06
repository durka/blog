---
layout: default
---
## Blog goes here

### Posts
{% for post in site.posts %}
- [{{ post.title }}]({{ post.url | prepend: site.github.url }}) ({{ post.date | date: "%b %d %Y" }})
{% endfor %}





