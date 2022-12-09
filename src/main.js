const {invoke} = window.__TAURI__.tauri;

let degreesRowElement;
let todayElement;
let dateElement;
let timeElement;
let cityElement;

async function get_cities(city) {
    const cities = await invoke("get_cities", {city});
}

async function get_degrees(lat, lon) {
    const degrees = await invoke("get_weather", {lat, lon});
    degrees.forEach((value, i) => {

        const temperature = value["temperature"]
        const symbol = value["icon"]
        const dayDate = new Date(value["time"])

        // Day container
        const dayContainer = document.createElement("div")
        dayContainer.className = "column padding-right"

        // Weather icon
        const iconElement = document.createElement("img")
        iconElement.src = `/assets/${symbol}.svg`
        iconElement.className = 'weather-icon'
        dayContainer.appendChild(iconElement)

        // Weather degrees
        const degreesElement = document.createElement("p")
        degreesElement.textContent = `${temperature}°`
        degreesElement.className = "degree-text"
        dayContainer.append(degreesElement)

        if (i === 0) {
            todayElement.append(dayContainer)

            const dayDate = new Date(value["time"])
            dateElement.textContent = dayDate.toLocaleString(undefined, {
                year: 'numeric',
                month: '2-digit',
                day: '2-digit',
                weekday: "long"
            })
            setInterval(runCurrentTime, 1000)
        } else {
            const dayElement = document.createElement("p")
            dayElement.textContent = dayDate.toLocaleString(undefined, {
                weekday: "long"
            })
            dayElement.className = "degree-text"
            dayContainer.prepend(dayElement)
            degreesRowElement.append(dayContainer)
        }
    })
}

function runCurrentTime() {
    const currentDate = new Date()
    timeElement.textContent = currentDate.toLocaleString(undefined, {
        hour: '2-digit',
        hour12: false,
        minute: '2-digit',
        second: '2-digit'
    })
}

window.addEventListener("DOMContentLoaded", () => {
    degreesRowElement = document.querySelector("#timeline");
    todayElement = document.querySelector("#today");
    dateElement = document.querySelector("#date");
    timeElement = document.querySelector("#time");
    cityElement = document.querySelector("#city");
    cityElement.textContent = "Split"
    get_cities("Split").then()
    get_degrees(43.5147, 16.4435).then()
});
