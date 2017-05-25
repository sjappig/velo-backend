// a stupid implementation of Velo stats listing
{
    const PLAYERS_URI = "/players.json"
    const GAMES_URI = "/games.json"

    const PLAYERS_TABLE = document.querySelector("#players > tbody")
    const GAMES_TABLE = document.querySelector("#games > tbody")


    function tr(...items) {
        const tr = document.createElement("tr")
        items.forEach(item => tr.appendChild( td(item) ))
        return tr
    }

    function td(str) {
        const td = document.createElement("td")
        td.innerText = str
        return td
    }


    function updatePlayers() {
        PLAYERS_TABLE.innerHTML = "";

        fetch(PLAYERS_URI)
            .then(response => response.json())
            .then(json => {
                json.players.forEach(player => {
                    PLAYERS_TABLE.appendChild( tr(player.name, player.elo) )
                })
            })
    }

    function updateGameHistory() {
        GAMES_TABLE.innerHTML = "";

        fetch(GAMES_URI)
            .then(response => response.json())
            .then(json => {
                json.games.forEach(game => {
                    GAMES_TABLE.appendChild( 
                        tr( game.winner_name,
                            game.loser_name,
                            game.duration,
                            game.start_time))
                })
            })
    }


    function init() {
        updatePlayers()
        updateGameHistory()
    }

    init()
}