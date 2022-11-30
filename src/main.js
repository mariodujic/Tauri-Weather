const {invoke} = window.__TAURI__.tauri;

let degreesRowElement;
let todayElement;
let dateElement;
let timeElement;

async function get_degrees() {
    const degrees = await invoke("get_weather");
    degrees.forEach((value, i) => {

        const temperature = value["temperature"]
        const symbol = value["icon"]

        // Day container
        const dayElement = document.createElement("div")
        dayElement.className = "column padding-right"

        // Weather icon
        const iconElement = document.createElement("img")
        iconElement.src = `/assets/${symbol}.svg`
        iconElement.className = 'weather-icon'
        dayElement.appendChild(iconElement)

        // Weather degrees
        const degreesElement = document.createElement("p")
        degreesElement.textContent = `${temperature}°`
        degreesElement.className = "degree-text"
        dayElement.append(degreesElement)

        if (i === 0) {
            todayElement.append(dayElement)

            const dayDate = new Date(value["time"])
            dateElement.textContent = dayDate.toLocaleString(undefined, {
                year: 'numeric',
                month: '2-digit',
                day: '2-digit',
                weekday: "long"
            })

            const currentDate = new Date()
            timeElement.textContent = currentDate.toLocaleString(undefined, {
                hour: '2-digit',
                hour12: false,
                minute: '2-digit',
                second: '2-digit'
            })
        } else {
            degreesRowElement.append(dayElement)
        }
    })
}

window.addEventListener("DOMContentLoaded", () => {
    degreesRowElement = document.querySelector("#timeline");
    todayElement = document.querySelector("#today");
    dateElement = document.querySelector("#date");
    timeElement = document.querySelector("#time");
    get_degrees().then()
});
