const backend_url = "http://localhost:8080";

function on_enter(event, f) {
    if (event.keyCode == 13) {
        f()
    }
}

async function create_player() {
    const response = await fetch(backend_url + "/player", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ name: document.getElementById("add_player_input").value }),
    });
    let p = document.getElementById("added_player");
    if (p == null) {
        p = document.createElement('p');
        p.id = "added_player";
        document.getElementById("add_player_div").appendChild(p);
    }
    if (response.ok) {
        const player = await response.json();
        p.innerHTML = `added player with id ${player.id} and name "${player.name}"`;
    } else {
        p.innerHTML = "error: " + await response.text();
    }
}

async function remove_player() {
    const id = document.getElementById("remove_player_input").value;
    let text = "error: id is empty";
    // check for empty string is required since the backend routing matches this as a different endpoint if the string is empty
    if (id.length > 0) {
        const response = await fetch(`${backend_url}/player/${id}`, { method: "DELETE" });
        if (response.ok) {
            const player = await response.json();
            text = `removed player "${player.name}", had id ${player.id}`;
        } else {
            text = "error: " + await response.text();
        }
    }
    let p = document.getElementById("removed_player");
    if (p == null) {
        p = document.createElement('p');
        p.id = "removed_player";
        document.getElementById("remove_player_div").appendChild(p);
    }
    p.innerHTML = text;
}

async function kill_player() {
    const response = await fetch(backend_url + "/kill", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
            killer: document.getElementById("killer_input").value,
            target: document.getElementById("target_input").value,
        }),
    });
    let p = document.getElementById("next_target");
    if (p == null) {
        p = document.createElement('p');
        p.id = "next_target";
        document.getElementById("kill_player_div").appendChild(p);
    }
    p.innerHTML = response.ok ? `kill successful, next target has id ${await response.text()}` : "error: " + await response.text();
}

async function start_game() {
    const response = await fetch(backend_url + "/start", { method: "POST" });
    let p = document.getElementById("game_control_text");
    if (p == null) {
        p = document.createElement('p');
        p.id = "game_control_text";
        document.getElementById("game_control_div").appendChild(p);
    }
    p.innerHTML = response.ok ? "game started" : "error: " + await response.text();
}

async function finish_game() {
    const response = await fetch(backend_url + "/finish", { method: "POST" });
    let p = document.getElementById("game_control_text");
    if (p == null) {
        p = document.createElement('p');
        p.id = "game_control_text";
        document.getElementById("game_control_div").appendChild(p);
    }
    p.innerHTML = response.ok ? "game finished" : "error: " + await response.text();
}

async function reset_game() {
    const response = await fetch(backend_url + "/reset", { method: "POST" });
    let p = document.getElementById("game_control_text");
    if (p == null) {
        p = document.createElement('p');
        p.id = "game_control_text";
        document.getElementById("game_control_div").appendChild(p);
    }
    p.innerHTML = response.ok ? "game reset" : "error: " + await response.text();
}

async function full_player_data() {
    const id = document.getElementById("player_data_input").value;
    let text = "error: id is empty";
    if (id != null) {
        const data_response = await fetch(backend_url + "/player/" + id);
        const status_response = await fetch(backend_url + "/player/status/" + id);
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
    let p = document.getElementById("player_data");
    if (p == null) {
        p = document.createElement('p');
        p.id = "player_data";
        document.getElementById("player_data_div").appendChild(p);
    }
    p.innerHTML = text;
}

async function game_data() {
    const response = await fetch(backend_url + "/game/data");
    let p = document.getElementById("game_control_text");
    if (p == null) {
        p = document.createElement('p');
        p.id = "game_control_text";
        document.getElementById("game_control_div").appendChild(p);
    }
    p.innerHTML = response.ok ? JSON.stringify(await response.json()) : "error: " + await response.text();
}
