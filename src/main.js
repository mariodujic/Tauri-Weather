const {invoke} = window.__TAURI__.tauri;

let degreesRowElement;
let todayElement;
let locationsElement;
let queryElement;
let resultsElement;

async function getLocations(location) {
    return await invoke("get_locations", {location});
}

async function getWeather(lat, lon) {
    return await invoke("get_weather", {lat, lon});
}

async function populateWeatherScreen(lat, lon, location) {
    const weather = await getWeather(lat, lon)

    const dateElement = document.createElement("div")
    populateDateElement()
    addTimeElement()
    addLocationElement(location)

    function populateDateElement() {
        dateElement.id = "date"
        dateElement.className = "date"
        todayElement.append(dateElement)
    }

    weather.forEach((weather, index) => {
        addDayElements(weather, index, dateElement)
    })
}

function addTimeElement() {
    const timeElement = document.createElement("p")
    timeElement.id = "time"
    timeElement.className = "time"
    const currentDate = new Date()
    setInterval(() => {
        timeElement.textContent = currentDate.toLocaleString(undefined, {
            hour: '2-digit',
            hour12: false,
            minute: '2-digit',
            second: '2-digit'
        })
    }, 1000)
    todayElement.append(timeElement)
}

function addLocationElement(location) {
    const locationsElement = document.createElement("p")
    locationsElement.id = "location"
    locationsElement.className = "location"
    locationsElement.textContent = location
    locationsElement.addEventListener('click', showLocationsScreen)
    todayElement.append(locationsElement)
}

function addDayElements(weather, index, dateElement) {

    const temperature = weather["temperature"]
    const symbol = weather["icon"]
    const dayDate = new Date(weather["time"])

    // Day container
    const dayContainer = document.createElement("div")
    dayContainer.className = "column padding-right"

    // Weather icon
    const iconElement = document.createElement("img")
    iconElement.src = `/assets/${symbol}.svg`
    iconElement.className = 'weather-icon'
    dayContainer.appendChild(iconElement)

    // Weather degrees
    const temperatureElement = document.createElement("p")
    temperatureElement.textContent = `${temperature}°`
    temperatureElement.className = "temperature"
    dayContainer.append(temperatureElement)

    if (index === 0) {
        todayElement.append(dayContainer)

        const dayDate = new Date(weather["time"])
        dateElement.textContent = dayDate.toLocaleString(undefined, {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            weekday: "long"
        })
    } else {
        const dayElement = document.createElement("p")
        dayElement.textContent = dayDate.toLocaleString(undefined, {
            weekday: "long"
        })
        dayElement.className = "temperature"
        dayContainer.prepend(dayElement)
        degreesRowElement.append(dayContainer)
    }
}

const debounce = function (fn, millis) {
    let timer;
    return function () {
        let context = this;
        let args = arguments;
        clearTimeout(timer);
        timer = setTimeout(() => {
            fn.apply(context, args);
        }, millis);
    }
}

function showLocationsScreen() {
    locationsElement.style.display = "block"
    clearWeatherContent()
}

function clearWeatherContent() {
    todayElement.replaceChildren()
    degreesRowElement.replaceChildren()
}

function showWeatherScreen() {
    locationsElement.style.display = "none"
    clearLocationsContent()
}

function clearLocationsContent() {
    queryElement.value = ""
    resultsElement.replaceChildren()
}

function searchLocation() {
    getLocations(queryElement.value).then(locations => {
        resultsElement.replaceChildren()
        locations.forEach(location => {
            const resultElement = document.createElement("p")
            resultElement.textContent = `${location["name"]}, ${location["admin1"]}, ${location["country"]}`
            resultElement.addEventListener('click', function () {
                const name = location["name"]
                const lat = parseFloat(location["lat"])
                const lon = parseFloat(location["lon"])
                populateWeatherScreen(lat, lon, name)
                    .then(() => showWeatherScreen())
                    .catch(reason => alert(reason))
            })
            resultsElement.appendChild(resultElement)
        })
    })
}

window.addEventListener("DOMContentLoaded", () => {
    degreesRowElement = document.querySelector("#timeline");
    todayElement = document.querySelector("#today");
    locationsElement = document.querySelector("#locations")
    queryElement = document.querySelector("#query")
    resultsElement = document.querySelector("#results")

    queryElement.addEventListener('input', debounce(searchLocation, 500));
});