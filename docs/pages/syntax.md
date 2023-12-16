# Syntax

Please note that Kitten is a compiler, which means that it transforms one language into another. It does not interpret the code directly. Code written in `.kitten` is converted into `.html` files, which can be read by browsers.

## Elements

In Kitten, you can create an element like this:

`index.kitten`:

```
div[]{"hi"}
```

This will be compiled to the following HTML:

`index.html`:

```
<div>hi</div>
```

In this example, `div` is the name of the element, `[]` is an empty section for attributes, and `{}` contains the children of this element. If the children are plain text, we surround it with `""` string quotes. If we want to nest another element, we can just insert it:

Here's an example of nested elements in Kitten:

`index.kitten`:

```
div[]{
    div[]{"hello"}
    div[]{"world"}
}
```

`index.html`:

```
<div>
    <div>hello</div>
    <div>world</div>
</div>
```

You can nest as many elements as you want.

## Attributes

In Kitten, you can add attributes to an element like this:

`index.kitten`:

```
a[href:/about; classname:link;]{"about"}
```

`index.html`:

```
<a href="/about" classname="link">about</a>
```

## Self-closed elements

Not every HTML element has a closing tag. To write such elements in Kitten, you just need to get rid of `{}`. Here's an example:

`index.kitten`:

```
img[src:/logo.png; alt:logo;]
```

`index.html`:

```
<img src="/logo.png"; alt="logo"; />
```

Please also note that Kitten is in the stage of early development and it may cause features and syntax to change drastically.
