const backend_url = "http://localhost:8080";

function on_enter(event, f) {
    if (event.keyCode == 13) {
        f()
    }
}

// returns null if no selection has been made
function get_game_id() {
    const selection = document.querySelector('input[name=game_selection]:checked');
    return selection == null ? null : selection.value;
}

function get_p(name) {
    let p = document.getElementById(name + "_text");
    if (p == null) {
        p = document.createElement('p');
        p.id = name + "_text";
        document.getElementById(name + "_div").appendChild(p);
    }
    return p;
}

async function create_player() {
    const game_id = get_game_id();
    const p = get_p("add_player");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }

    const response = await fetch(`${backend_url}/games/${game_id}/players`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ name: document.getElementById("add_player_input").value }),
    });
    if (response.ok) {
        const player = await response.json();
        p.innerHTML = `added player with id ${player.id} and name "${player.name}"`;
    } else {
        p.innerHTML = "error: " + await response.text();
    }
}

async function remove_player() {
    const game_id = get_game_id();
    const player_id = document.getElementById("remove_player_input").value;
    const p = get_p("remove_player");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }

    let text = "error: id is empty";
    // check for empty string is required since the backend routing matches this as a different endpoint if the string is empty
    if (player_id.length > 0) {
        const response = await fetch(`${backend_url}/games/${game_id}/players/${player_id}`, { method: "DELETE" });
        if (response.ok) {
            const player = await response.json();
            text = `removed player "${player.name}", had id ${player.id}`;
        } else {
            text = "error: " + await response.text();
        }
    }
    p.innerHTML = text;
}

async function kill_player() {
    const game_id = get_game_id();
    const p = get_p("kill_player");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }

    const response = await fetch(`${backend_url}/games/${game_id}/kill`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
            killer: document.getElementById("killer_input").value,
            target: document.getElementById("target_input").value,
        }),
    });
    p.innerHTML = response.ok ? `kill successful, next target has id ${await response.text()}` : "error: " + await response.text();
}

async function start_game() {
    const game_id = get_game_id();
    const p = get_p("game_control");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }

    const response = await fetch(`${backend_url}/games/${game_id}/start`, { method: "POST" });
    p.innerHTML = response.ok ? "game started" : "error: " + await response.text();
}

async function finish_game() {
    const game_id = get_game_id();
    const p = get_p("game_control");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }
    const response = await fetch(`${backend_url}/games/${game_id}/finish`, { method: "POST" });
    p.innerHTML = response.ok ? "game finished" : "error: " + await response.text();
}

async function reset_game() {
    const game_id = get_game_id();
    const p = get_p("game_control");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }

    const response = await fetch(`${backend_url}/games/${game_id}/reset`, { method: "POST" });
    p.innerHTML = response.ok ? "game reset" : "error: " + await response.text();
}

async function full_player_data() {
    const game_id = get_game_id();
    const p = get_p("player_data");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }

    const player_id = document.getElementById("player_data_input").value;
    let text = "error: id is empty";
    if (player_id != null) {
        const data_response = await fetch(`${backend_url}/games/${game_id}/players/${player_id}`);
        const status_response = await fetch(`${backend_url}/games/${game_id}/players/${player_id}/status`);
        if (data_response.ok) {
            const player_data = await data_response.json();
            text = `player "${player_data.name}" has ${player_data.lives} lives, ${player_data.kills} kills, and id ${player_data.id}`;
        } else {
            text = "error while fetching data: " + await data_response.text();
        }
        text += "<br>";
        if (status_response.ok) {
            text += "circle status:<br>";
            const status = await status_response.json();
            console.log(JSON.stringify(status));
            Object.keys(status).sort().forEach(i => {
                const circle_status = status[i];
                text += `${i}: ${Object.hasOwn(circle_status, "Alive") ? "player is alive with target id " + circle_status.Alive.target : Object.hasOwn(circle_status, "Dead") ? "player has been killed by player with id " + circle_status.Dead.killer : "invalid status"}`;
                text += "<br>";
            });
        } else {
            text += "error while fetching status: " + await status_response.text();
        }
    }
    p.innerHTML = text;
}

async function game_data() {
    const game_id = get_game_id();
    const p = get_p("game_control");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }

    const response = await fetch(`${backend_url}/games/${game_id}`);
    p.innerHTML = response.ok ? JSON.stringify(await response.json()) : "error: " + await response.text();
}

async function import_game() {
    const files = document.getElementById("file_import_input").files;
    const p = get_p("game_export");

    if (files == null) {
        p.innerHTML = "error fetching files from html element"
    } else if (files.length == 0) {
        p.innerHTML = "please select a file first"
    } else {
        const response = await fetch(`${backend_url}/games/import`, {
            method: "PUT",
            body: files[0],
        });
        p.innerHTML = response.ok ? "import successful" : "error: " + await response.text();
    }
}

async function export_game() {
    const game_id = get_game_id();
    const p = get_p("game_export");
    if (game_id == null) {
        p.innerHTML = "please select a game first";
        return;
    }
    window.location.href = `${backend_url}/games/${game_id}/export`
}

async function create_game_selection() {
    const response = await fetch(`${backend_url}/games`);
    const selection_div = document.getElementById("game_selection_div");
    if (!response.ok) {
        p = document.createElement("p");
        p.id = "game_selection_text";
        p.innerHTML = "error fetching games: " + await response.text();
        selection_div.appendChild(p);
        return;
    }
    (await response.json()).forEach(game => {
        const radio_button = document.createElement("input");
        radio_button.type = "radio";
        radio_button.id = "selection-" + game.id;
        radio_button.name = "game_selection";
        radio_button.value = game.id;

        const label = document.createElement("label");
        label.for = radio_button.id;
        label.innerHTML = `${game.name} (in phase ${game.phase} with ${game.players.length} players)`;

        selection_div.appendChild(radio_button);
        selection_div.appendChild(label);
        selection_div.appendChild(document.createElement("br"));
    });
}

// call on page load
create_game_selection();
