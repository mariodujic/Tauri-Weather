const {invoke} = window.__TAURI__.tauri;

let degreesRowElement;
let todayElement;
let dateElement;
let timeElement;
let cityElement;
let locationsElement;
let queryElement;
let resultsElement;

async function get_locations(location) {
    return await invoke("get_locations", {location});
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

const debounce = function(fn, millis) {
    let timer;
    return function() {
        let context = this;
        let args = arguments;
        clearTimeout(timer);
        timer = setTimeout(() => {
            fn.apply(context, args);
        }, millis);
    }
}

function findLocation() {
    get_locations(queryElement.value).then(locations => {
        resultsElement.replaceChildren()
        locations.forEach(location => {
            const resultElement = document.createElement("p")
            resultElement.textContent = `${location["name"]}, ${location["admin1"]}, ${location["country"]}`
            resultElement.addEventListener('click', function () {
                const name = location["name"]
                cityElement.textContent = name
                const lat = parseFloat(location["lat"])
                const lon = parseFloat(location["lon"])
                get_degrees(lat, lon)
                    .then(() => {locationsElement.style.display = "none"})
                    .catch(reason => alert(reason))
            })
            resultsElement.appendChild(resultElement)
        })
    })
}

window.addEventListener("DOMContentLoaded", () => {
    degreesRowElement = document.querySelector("#timeline");
    todayElement = document.querySelector("#today");
    dateElement = document.querySelector("#date");
    timeElement = document.querySelector("#time");
    cityElement = document.querySelector("#city");
    locationsElement = document.querySelector("#locations")
    queryElement = document.querySelector("#query")
    resultsElement = document.querySelector("#results")

    queryElement.addEventListener('input', debounce(findLocation, 500));
});