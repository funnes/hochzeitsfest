Um die Einladung anzunehmen oder abzulehnen, editiere die `gästeliste.toml` Datei und eröffne einen Pull Request.
Hier ein Beispiel:

``` diff
 [[guests]]
 name = "Foo Bar"
+attending = ["afternoon", "dinner"]
```

Der Wert von `attending` muss eine Teilmenge von `["afternoon", "dinner", "hike"]` sein.
Setze `attending` auf `[]` um die Einladung abzulehnen.

Wenn die Einladung mehrere Personen betrifft muss der Eintrag so aussehen:

``` diff
 [[guests]]
 name = "Foo and Bar"
+attending.Foo = ["afternoon", "dinner"]
+attending.Bar = ["afternoon"]
```
