# ECSM

This project is under development,
A proof of concept: [Ecsm-py](https://github.com/9elt/Ecsm-py)


### html

<table>

<tr>
<td>

<img width=322/>
source

</td>
<td>

<img width=322/>
output

</td>
</tr>

<tr>
<td>


```html
<body>

  <div>
    <h1 handle_state="test">click here</h1>
    <p class="content">hello</p>
  </div>

</body>
```

</td>

<td>

```html
<body>

  <input
    class="ECSM-state"
    id="ECSM-bool-ID_test"
    type="checkbox"
  />

  <div>
    <label for="ECSM-bool-ID_test">
      <h1>click here</h1>
    </label>
    <p class="content">hello</p>
  </div>

</body>
```

</td>
</tr>


</table>

### css

<table>

<tr>
<td>

<img width=322/>
source

</td>
<td>

<img width=322/>
output

</td>
</tr>

<tr>
<td>


```css
test:active .content {
  color: #f00
}
```

</td>

<td>


```css
.ECSM-state { display: none !important }

#ECSM-bool-ID_test:checked~* .content {
  color: #f00
}
```

</td>
</tr>


</table>