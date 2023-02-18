# ECSM
> This project is under development

A framework to make the development of html and css only websites easy.

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

    <p handle_state="test">
      click here
    </p>

    <p class="content">hello</p>

  </div>

</body>
```

</td>

<td width="50">

```html
<body>

  <input
    class="ECSM-state"
    id="ECSM-boolean-ID-test"
    type="checkbox"
  />

  <div>

    <label for="ECSM-boolean-ID-test">
      <p>click here</p>
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
  color: #F00
}
```

</td>

<td>

```css
#ECSM-boolean-ID-test:checked~* .content {
  color: #F00
}
```

</td>
</tr>


</table>
