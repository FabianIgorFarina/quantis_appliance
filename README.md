# Quantis Appliance Simulator

## Installation

Dieses Programm ist die teilweise Simulation der API des Quantis Appliance 2.0.

Um es ausführen zu können ist Rust, sowie die OpenSSL Entwicklerbibliotheken notwendig.
Installation von OpenSSL Entwicklerbibliotheken auf Ubuntu oder Debian:

```
sudo apt update
sudo apt install libssl-dev

```

Die anschließende Installation von Rust ist auf deren Website beschrieben:
[Rust Installation](https://www.rust-lang.org/tools/install)

## Hostname festlegen (optional)

Der Hostname des beigefügten Zertifikats ist "server1" und es wurde von unserer eigenen CA (ca.crt) unterzeichnet. Damit dies bei einer Abfrage akzeptiert wird, muss der Hostname entsprechend sein:
```
sudo hostname server1
```
Bis zum nächsten Neustart ist der Hostname der Maschine dann "server1".

Für ein permanentes Setzen des Hostnames kann auch folgendes verwendet werden:
```
sudo hostnamectl set-hostname server1
```

Damit abfragen richtig geleitet werden muss der Hostname noch mit der lokalen IP-Adresse in Verbindung gebracht werden. Dazu muss der Datei /etc/hosts folgende Zeile hinzugefügt werden:
```
127.0.0.1 server1
```

## Programm starten

Das Programm sollte sich nun starten lassen:
```
cargo run
```
Der Listener für die Abfragen läuft auf Port 8080. 

## API

Abgefragt werden können die Datentypen "int", "short", "double" und "float". Die Parameter der Query sind "min" (untere Schranke), "max" (obere Schranke) und "quantity" (Anzahl der gewünschten Zufallszahlen).
Beispiele zum Aufruf mit curl:
```
curl -k "https://localhost:8080/api/2.0/int" # Abruf ohne vorherige Anpassung des Hosnames (unsicher)
curl --cacert ca.crt "https://server1:8080/api/2.0/int" # Die Abfrage muss aus dem Ordner erfolgen, in dem sich ca.crt befindet, ansonsten muss der Pfad dorthin angegeben werden.
curl --cacert ca.crt "https://server1:8080/api/2.0/double?min=-10&max=12&quantity=5"
```
