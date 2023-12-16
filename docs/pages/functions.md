# Functions

Functions are used for making javascript calculations inside `.kitten` file.

`index.kitten`

```
div[]{
    (){
        let x = 5;
        return x + 5;
    }
}
```

`index.html`

```
<div>10</div>
```

Functions must return something or error will be thrown. Also, you cant return `Kitten` elements for now.
