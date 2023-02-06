# ECSM

> This project is under development

A tool to easily manage state in html and css only websites

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

    <p handle_state="onemore:akey">
      click here
    </p>

    <p handle_state="onemore:default">
      click here
    </p>

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
    id="ECSM-boolean-ID-test"
    type="checkbox"
  />

  <input
    class="ECSM-state"
    id="ECSM-selection-ID-onemore-KEY-akey"
    type="radio"
  />

  <input
    class="ECSM-state"
    id="ECSM-selection-ID-onemore-KEY-default"
    type="radio"
    checked
  />

  <div>

    <label for="ECSM-boolean-ID-test">
      <p>click here</p>
    </label>

    <label for="ECSM-selection-ID-onemore-KEY-akey">
      <p>click here</p>
    </label>

    <label for="ECSM-selection-ID-onemore-KEY-default">
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

onemore:akey .content {
  color: #FFF
}

onemore:default .content {
  color: #000
}
```

</td>

<td>


```css
#ECSM-boolean-ID-test:checked~* .content {
  color: #F00
}

#ECSM-selection-ID-onemore-KEY-akey:checked~* .content {
  color: #FFF
}

#ECSM-selection-ID-onemore-KEY-default:checked~* .content {
  color: #000
}
```

</td>
</tr>


</table>