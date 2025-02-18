#let front(guest, swap-title: false) = [
  #set align(center)
  #set text(
    font: "Libertinus Serif",
    fill: rgb("#424b4d"),
    size: 10pt,
  )
  #show heading: set text(
    size: 2em,
    font: "Tangerine",
    fill: rgb("#4e6e65"),
  )
  #show heading.where(level: 1): set text(size: 1.5em)

  #let qrheight = 1.5cm

  #let is-multiple = " und " in guest.name
  #let is-turnverein = "Turnverein" in guest.name

  = #if swap-title { "Johannes & Petra" } else { "Petra & Johannes" }

  #v(1em)

  Wir laden
  #{
    if is-multiple or is-turnverein { "euch" } else { "dich" }
    ","
    if is-turnverein { " den" }
    [\ ]
  }
  #text(size: 1.3em, guest.name),\
  herzlich zu unserem Hochzeitsfest ein.\

  #text(size: 1.1em, [*2025-07-19 14:00 Uhr*])\
  Chräen, Krähenweg 9, 8413 Neftenbach

  == Anmeldung

  Bitte gebt uns (#link("tel:+41 79 884 75 55")) bis *2025-03-14* Bescheid, ob ihr beim Apéro dabei seid.\

  == Trauzeugen

  Marina Neyer (#link("tel:+43 676 9677738"))\
  Nick Witzig (#link("tel:+41 79 323 30 94"))

  == Geschenke

  Anstelle eines Geschenks freuen wir uns über eine Spende an eine gemeinnützige Organisation wie Greenpeace.

  #image("img/greenpeace.svg", height: qrheight)
]

#let back = context {
  set align(center)
  let spacing = 5mm
  set page(margin: 0cm, header: none, footer: none)
  set par(spacing: 0cm)

  // page.width = 3 * spacing + 2 * small-photo-width
  // page.height = 3 * spacing + large-photo-height + small-photo-width
  let small-photo-width = (page.width - 3 * spacing) / 2
  let large-photo-height = page.height - 3 * spacing - small-photo-width
  let large-photo-width = page.width - 2 * spacing

  v(spacing)
  box(image("img/IMG_20240811_102902_836.jpg", height: large-photo-height, width: large-photo-width))
  parbreak()
  v(spacing)
  box(image("img/IMG_20240205_144332_849.jpg", width: small-photo-width))
  h(spacing)
  box(image("img/IMG_20240824_204122_040.jpg", width: small-photo-width))
  v(spacing)
}

#set page("a5")

#let guests = toml("./gästeliste.toml").guests

// count invited people
// #guests.fold(0, (acc, g) => acc + g.name.split(regex("(,? und |,)")).len())

#let guests = guests.filter(g => g.is-invited-by-paper)

#front(guests.last())
#back

// prepare for printing
// #for (g1, g2) in guests.chunks(2) {
//   front(g1, swap-title: true)
//   front(g2, swap-title: false)
//   back
//   back
// }
// pdfjam --nup 2x1 --landscape --paper a4paper /tmp/einladung.pdf -o /tmp/out.pdf

// Local Variables:
// typst-ts-watch-options: "/tmp/einladung.pdf --font-path fonts --open"
// End:

