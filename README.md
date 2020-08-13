# Sigma

> A simple static site generator built with Rust.

### Usage:

1. Create a `.zip` archive containing the following structure:

```
.
├───assets 
│   │   
│   ├─sass
│   │ │ 
│   │ ├─__partial.scss
│   │ │ 
│   │ └─style.scss
│   │
│   ├─js
│   │ │ 
│   │ └─script.js
│   └─img
│     │ 
│     └─banner.img
│   
├───posts 
│   └─post.md 
│   
└───templates
    └─post.hbs
```

1. Put the `.zip` archive into the root directory of the folder.

2. Execute `cargo run your_archive_name.zip`

---

### Example:

> An example `example.zip` file has been provided for testing. 

Example Markdown Post:

`./posts/post.md`
```
---
template: post
author: Test User
date: 1970-01-01
title: First Post
---
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus sed congue turpis, sit amet posuere neque. Nam vehicula posuere tristique. Mauris et diam sed dolor faucibus fermentum. Ut commodo justo nec nulla auctor, eget viverra velit porttitor. Etiam auctor orci eget nisi cursus, non euismod felis imperdiet. Vivamus sagittis sapien vitae ullamcorper mollis. Fusce fermentum libero odio, in semper lacus vehicula at. Vestibulum sit amet lectus tempus, efficitur dolor in, sagittis enim. Nunc imperdiet risus ex, eget viverra quam egestas sit amet. Proin ac mi eu ipsum rutrum eleifend nec tempor nibh.
```

Example Handlebars Template:

`./templates/post.hbs`

```hbs
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{ title }}</title>
  <link rel="stylesheet" href="../css/style.css">
</head>

<body>
  <h1>{{ title }}</h1>
  <h4>by {{ author }}</h4>
  <p>{{ date }}</p>
  <hr>
  {{{ content }}}
</body>

</html>
```