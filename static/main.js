// a stupid implementation of Velo stats listing
{
    const PLAYERS_URI = "/players.json"
    const GAMES_URI = "/games.json"

    const PLAYERS_TABLE = document.querySelector("#players > tbody")
    const GAMES_TABLE = document.querySelector("#games > tbody")

    const _players = {}

    const tr = function(...items) {
        const tr = document.createElement("tr")
        items.forEach(item => tr.appendChild( td(item) ))
        return tr
    }

    const td = function(str) {
        const td = document.createElement("td")
        td.innerText = str
        return td
    }

    const getPlayerName = function(id) {
        if (id in _players) {
            return _players[id].name
        }
        console.error(`player ${id} not found!`)
        return id.substring(0,6)
    }


    /// GLOBAL FUNCTIONS

    function updatePlayers() {
        PLAYERS_TABLE.innerHTML = "";

        return fetch(PLAYERS_URI)
            .then(response => response.json())
            .then(json => {
                json.players.forEach(player => {
                    _players[player.id] = player
                    PLAYERS_TABLE.appendChild( tr(player.name, player.elo) )
                })
            })
    }

    function updateGameHistory() {
        GAMES_TABLE.innerHTML = "";

        return fetch(GAMES_URI)
            .then(response => response.json())
            .then(json => {
                json.games.forEach(game => {
                    GAMES_TABLE.appendChild( 
                        tr( getPlayerName(game.winner),
                            getPlayerName(game.loser),
                            game.duration,
                            game.start_time))
                })
            })
    }

    function update() {
        updatePlayers().then(updateGameHistory)
    }

    update()
}