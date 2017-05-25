// a stupid implementation of Velo stats listing
{
    const PLAYERS_URI = "/player"
    const GAMES_URI = "/game"

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

    /**
     * Zero-pad a positive integer to a (minimum) width of 2.
     */
    const pad2 = function(i) {
        if (i < 10 && i >= 0) {
            return `0${i}`
        }
        return i
    }

    /**
     * Format date like "2017-05-15 15:42" (in browser timezone)
     * 
     * @param {*} str date in a format understood by the browser, eg. ISO 8601
     */
    const formatDate = function(str) {
        const d = new Date(str);
        return `${d.getFullYear()}-${pad2(d.getMonth() + 1)}-${pad2(d.getDate())} ${d.getHours()}:${d.getMinutes()}`
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
                            formatDate(game.start_time)))
                })
            })
    }

    function update() {
        updatePlayers().then(updateGameHistory)
    }

    update()
}
