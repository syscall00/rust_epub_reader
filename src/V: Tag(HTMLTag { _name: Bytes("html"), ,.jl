V: Tag(HTMLTag { _name: Bytes("html"), , _children: InlineVec(InlineVec<5 items>), _raw: Bytes("<html>\n    <head>\n        <!-- head definitions go here -->\n    </head>\n    <body>\n        <!-- the content goes here -->\n        <p>Test <b>dei</b><i>tag</i></p>\n    </body>\n</html>") })
Tag Bytes("html")

V: Tag(HTMLTag { _name: Bytes("head"), _children: InlineVec(InlineVec<3 items>), _raw: Bytes("<head>\n        <!-- head definitions go here -->\n    </head>") })
Tag Bytes("head")

V: Comment(Bytes("<!-- head definitions go here -->"))

V: Tag(HTMLTag { _name: Bytes("body"), _children: InlineVec(InlineVec<5 items>), _raw: Bytes("<body>\n        <!-- the content goes here -->\n        <p>Test <b>dei</b><i>tag</i></p>\n    </body>") })
Tag Bytes("body")

V: Comment(Bytes("<!-- the content goes here -->"))

V: Tag(HTMLTag { _name: Bytes("p"), _children: InlineVec(InlineVec<3 items>), _raw: Bytes("<p>Test <b>dei</b><i>tag</i></p>") })
Tag Bytes("p")

V: Raw(Bytes("Test "))
V: Tag(HTMLTag { _name: Bytes("b"), _children: InlineVec(InlineVec<1 items>), _raw: Bytes("<b>dei</b>") })
Tag Bytes("b")

V: Raw(Bytes("dei"))
V: Tag(HTMLTag { _name: Bytes("i"), _children: InlineVec(InlineVec<1 items>), _raw: Bytes("<i>tag</i>") })
Tag Bytes("i")

V: Raw(Bytes("tag"))
