# GUI for Rust

Little an simple gui library for rust inspired by react.
gui and uses cairo as backend library for rendering.
But this will may change in the future.

## Konzept

Ein Programm besteht aus mehreren Komponenten. Jedes dieser komponenten kann dafür sorgen, das
es gecachd wird, und das es einen Zustand besitzt.

Der Baum mit den Componenten, wird allerdings nicht beahalten, sondern wenn benötigt lazy
erstellt. Das heißt es wird kein zusätzlicher Speicherplatz benötigt wenn sich nichts ändert,
und Änderungen können schnell und einfach geschehen.

Jedes element besitzt eine Render-funktion. In dieser werden die Elemente initialisiert und an
den context geschickt. Hierbei bekommt jedes Objekt eine ID um.

## Events
vom Context selbst werden Events bereitgestellt, die von jedem element aboniert werden könne.
Wenn eines dieser Events auslöst wird die Render-funktion des gesuchten elements aufgerufen, und
das Event gehandled.

Außerdem können Componenten selbs event bereitstellen. Diese werden vom Context verwalted und
gehen immer vom Child-Element zum Parent-Element, so das das Parent element diese behandeln kann.
Events können mit der Funktion `ctx.emit(event: Event);` ausgelößst werden. Sie werden
übertragen, so bald die Funktion beendet ist. Als Event-Type sind besonders Enums geeignet,
da ein Widget verschiedene Events haben könnte und so differenziert werden kann.

## Performance
Parts of the UI are cached for faster rendering.
Where this is necessary is determined automatically by using timing
functions to find out what takes how long.

## Zeichnen
Es wird immer von oben nach unten gezeichnet das heißt bei der Anordnung ist die Reihenfolge zu
beachten. Um die Position einzelner Komponenten zu bestimmen kann die Parent funktion deren Position
bestimmen.

## TODO:
 - make component system work
 - build some basic components
   - button
   - radio
   - label
   - lineEdit
   - numberInput
   - multilineEdit
   - Tabs
   - List
   - Box-Layout
 - create macro for easier interaction
 - animation
 - caching
 - state management
 - documentation
 - theming
 - publish

## Screenshots

![all_widgets example](./screenshot_all_widgets.png)
