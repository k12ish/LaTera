# LaTera

LaTera is a tera-based template engine. It changes Tera's syntax to be more LaTeX compatible. Here's an example comparing Tera on HTML vs LaTera on LaTeX.

```jinja2
<title>{% block title %}{% endblock title %}</title>
<ul>
{% for user in users %}
  <li><a href="{{ user.url }}">{{ user.username }}</a></li>
{% endfor %}
</ul>
{# a comment #}
```

```latex
\section{ <# block section #><# endblock section #> }
\begin{itemize}
  <# for user in users #>
    \item \href{ << user.url >> }{ << user.username >> }
  <# endfor #>
\end{itemize}
<% a comment %>
```

## Changelog

- [X] Changed block syntax

```diff
-variable_start = { "{{-" | "{{" }
-variable_end   = { "-}}" | "}}" }
+variable_start = { "<<-" | "<<" }
+variable_end   = { "->>" | ">>" }
-tag_start      = { "{%-" | "{%" }
-tag_end        = { "-%}" | "%}" }
+tag_start      = { "<#-" | "<#" }
+tag_end        = { "-#>" | "#>" }
-comment_start  = { "{#-" | "{#" }
-comment_end    = { "-#}" | "#}" }
+comment_start  = { "<%-" | "<%" }
+comment_end    = { "-%>" | "%>" }
```

- [ ] Rewrite tests
- [ ] Escape text for LaTeX, not HTML