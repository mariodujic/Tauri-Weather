const {invoke} = window.__TAURI__.tauri;

let weatherElement;
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
    updateTime()
    setInterval(updateTime, 1000)
    todayElement.append(timeElement)

    function updateTime() {
        const currentDate = new Date()
        timeElement.textContent = currentDate.toLocaleString(undefined, {
            hour: '2-digit',
            hour12: false,
            minute: '2-digit',
            second: '2-digit'
        })
    }
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

    const dayContainer = document.createElement("div")
    dayContainer.className = "column padding-right"

    const iconElement = document.createElement("img")
    iconElement.src = `/assets/${symbol}.svg`
    iconElement.className = 'weather-icon'
    dayContainer.appendChild(iconElement)

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
    weatherElement.style.display = "none"
    todayElement.replaceChildren()
    degreesRowElement.replaceChildren()
}

function showWeatherScreen() {
    locationsElement.style.display = "none"
    weatherElement.style.display = "flex"
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
                    .then(() => {
                        cacheSelectedLocation(location)
                        showWeatherScreen()
                    })
                    .catch(reason => alert(reason))
            })
            resultsElement.appendChild(resultElement)
        })
    })
}

const LOCATION_KEY = "weather_location_key"

function cacheSelectedLocation(location) {
    const locationValue = JSON.stringify(location)
    localStorage.setItem(LOCATION_KEY, locationValue)
}

function getCachedLocation() {
    const locationValue = localStorage.getItem(LOCATION_KEY)
    const location = JSON.parse(locationValue)
    return location
}

function loadCachedLocation() {
    const location = getCachedLocation()
    if (location !== undefined) {
        const name = location["name"]
        const lat = parseFloat(location["lat"])
        const lon = parseFloat(location["lon"])
        populateWeatherScreen(lat, lon, name)
            .then(() => cacheSelectedLocation(location))
            .catch(reason => alert(reason))
        showWeatherScreen()
    }
}

window.addEventListener("DOMContentLoaded", () => {
    weatherElement = document.querySelector("#weather");
    degreesRowElement = document.querySelector("#timeline");
    todayElement = document.querySelector("#today");
    locationsElement = document.querySelector("#locations")
    resultsElement = document.querySelector("#results")
    queryElement = document.querySelector("#query")
    queryElement.addEventListener('input', debounce(searchLocation, 500));

    loadCachedLocation()
});