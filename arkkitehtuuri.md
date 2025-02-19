# Arkittehtuuri

Tässä dokumentissa eritellään, millainen rakenne projektilla on tarkoitus olla.
Tarkempi tekninen toteutus on kuvattu erikseen `server`- ja `client`-
hakemistoissa.

## Yleistä

Projekti toteutetaan hyödyntäen Docker-kontteja. Näin mahdollistetaan
kohtuullisen kevyt mutta ylläpidettävä ja siirrettävä julkaisuympäristö,
jossa on mahdollista eriyttää paikallinen kehitysversio, testausversio ja
julkaistu varsinainen versio toisistaan.

Kaikki fuksiseikkailu-sovelluksen koodi on julkaistu Github-repositoriossa,
ja se julkaistaan avoimen lähdekoodin MIT-lisenssillä.

Sovellus on tarkoitus voida julkaista missä tahansa ympäristössä, joka tukee
docker-kontteja. Julkaisualusta on siis osa yksittäisen instanssin teknistä
toteutusta. Todennäköisesti lähempänä julkaisua tullaan kuitenkin kirjoittamaan
ohje sovelluksen käyttöönotolle Heroku-ympäristössä.

Lähtökohtaisesti sovellus on pyritty toteuttamaan aikaa kestävillä ja vakailla
kirjastoilla ja tekniikoilla. Näin sen ylläpitämisen ei pitäisi vaatia spesifiä
teknistä osaamista, ja haltuunotto on yksinkertaista.

## Client
Client-puoli on verkkosovellus, joka on kirjoitettu Typescriptillä ja puhtaalla
CSS:llä, käyttäen Vite-frameworkia. Mahdollisesti jatkossa tullaan ottamaan
mukaan kirjastoja, joilla kommunikaatio APIn ja tietokantataulujen ja
typescriptin välillä onnistuu automaattisesti.

Kartta-ominaisuus on tarkoitus toteuttaa Leaflet.js -kirjastolla.

Tarkoituksena on olla käyttämättä liian montaa valmista kirjastoa, jotta
varmistetaan sovelluksen pitkä-ikäisyys.

## Server
Server-puoli on Pythonilla kirjoitettu FastAPI-sovellus, joka tarjoaa
REST-rajapinnan PostgreSQL-tietokantaan, pääsääntöisesti JSON-muotoisena.

Tietokanta tukee migraatioita, jotta eri kehitysvaiheissa on mahdollista muokata
tietokannan tauluja melko vapaasti.
