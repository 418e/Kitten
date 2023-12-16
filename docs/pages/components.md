# Components

Creating elements as components to sharing across files is very simple in Kitten:

`hello.kitten`

```
div[key:$hello;]{"Hello!"}
```

`index.kitten`

```
import[from:hello; element:$hello; as:Hello; ]

div[]{
    Hello[]{}
}
```

`index.html`

```
<div>
    <div key="$hello">Hello!</div>
</div>
```

In the example above, we imported an element with a `key` attribute value of `hello`. To do this, we first assigned a `key` attribute to the element. Then, in the file where we wanted to import the element, we added an `import[]` element with the following attributes:

- `from` - The location of the `.kitten` file.
- `element` - The value of the `key` attribute we are importing.
- `as` - The name we will use to display the element.
