Funktionsbeschreibung des Picos
Der pico ist in states auf geteilt

In jedem state to ther pico etwas. 

Der zu stand des States läuct asynchron zur Wifi verindung und zum Live Bit


## Lifebit 

das Lifebit wird getoggelt jede 1s um verzustellen das das Programm normal läuft
sollte das Lifebit die Frequenz erhöhen oder verringern dann liegt ein Fehler vor.


## States 

In der States arbeitet der Pico eine Abläufe und Prgramme ab. 

- Init => Er fährt hoch und "orientiert sich". Wifi connection wird aufgebaut und das erstmal über die API daten abgefragt 

- Idle => im idle state hat der Pico bereits Daten gesendet und wartet bis die Zeit abgelaufen ist damiut er über die Api erneut daten abfragen kann

- Update => Daten werden geholt und der Screen wird damit refreshed

- Error => Das Programm wird nicht in den panic geschickt sondern es wird auf dem Screen eine Fehlermeldung angezeigt


Der Ausgang für alle States ist der Idle.
Im Error fall wird danach überprüft was fehlt und dieser Fehler wird "behoben" -> Daten neu erzeugt oder ähnliches

Wenn die Fehler behoben sind dann wird er zurück in den Init gesetzt. 

Sollte kein Fehler behoben werden können, dann wird der Init nochmal ausgelöst



#### Init State

#### Idle State

#### Update State

#### Error State





