let categories = [];/* [
    { name: "Application", components: [
        { name: "Reviewer UI", progress: 0.0 },
        { name: "Publisher UI", progress: 0.0 },
        { name: "Administrator UI", progress: 0.0 },
        { name: "Bots", progress: 0.0 },
    ] },
    { name: "Presentation", components: [
        { name: "REST API", progress: 0.1 },
        { name: "Bot API", progress: 0.0 },
    ] },
    { name: "Business Logic", components: [
        { name: "Session Controller", progress: 0.0 },
        { name: "Document Controller", progress: 0.0 },
    ] },
    { name: "Persistence", components: [
        { name: "Encryption Controller", progress: 0.0 },
        { name: "User Controller", progress: 0.0 },
    ] },
    { name: "Data", components: [
        { name: "Data Controller", progress: 0.0 },
    ] },
    { name: "Physical", components: [
        { name: "Database", progress: 0.0 }
    ] },
]*/

function makeComponent(component) {
    let componentDiv = $(`<div class="component"></div>`)

    let progress_bar = $(`<div class="progress-bar" style="width: ${component.progress * 100}%"></div>`);
    componentDiv.append(progress_bar);

    let title = $(`<p id="title">${component.name}</p>`);
    componentDiv.append(title);

    return componentDiv
}

function makeCategory(category) {
    let categoryDiv = $(`<div class="category"></div>`)
    let title = $(`<p id="title">${category.name}</p>`);
    categoryDiv.append(title);

    let componentList = $(`<div class="component-list"></div>`);
    category.components
            .map(makeComponent)
            .forEach(componentDiv => componentList.append(componentDiv));

    categoryDiv.append(componentList);

    return categoryDiv;
}

function main() {
    let container = $('<div class="container"></div>')

    categories.map(makeCategory)
              .forEach(categoryDiv => container.append(categoryDiv))

    $('body').append(container);
}

function fetchData() {
    $.getJSON("http://127.0.0.1:3000/progress.json", function(data) {
        categories = data;

        main();
    });
}


$(document).ready(fetchData)