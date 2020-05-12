function getPageHeights() {
    var heights = new Object();
    heights.html = document.getElementById("html").getBoundingClientRect().height;
    //console.log("html = " +heights.html);
    heights.body = document.getElementById("body").getBoundingClientRect().height;
    //console.log("body = " +heights.body);
    heights.header = document.getElementById("header").getBoundingClientRect().height;
    //console.log("header = " +heights.header);
    heights.main = document.getElementById("main").getBoundingClientRect().height;
    //console.log("main = " +heights.main);
    heights.footer = document.getElementById("footer").getBoundingClientRect().height;
    //console.log("footer = " +heights.footer);
    return heights;
}

// Stick the footer at the bottom of the page if the combined height of header, main and html (the body) are not the same as the html
function stickFooterAtBottom() {
    var heights = getPageHeights();
    // Check
    if (heights.html > heights.body) {
        var footer_vertical_position = heights.html - heights.footer;
        //console.log("footer new vertical position: "+footer_vertical_position);
        document.getElementById("footer").style.top = footer_vertical_position+"px";
    };
}

// Resize the main to fill the screen if the body is smaller than the screen
function enlargeMain() {
    var heights = getPageHeights();
    if (heights.html > heights.body) {
        var main_height = heights.html - heights.header - heights.footer;
        //console.log("main new height: "+main_height);
        document.getElementById("main").style.height = main_height+"px";
    }
}

// Change theme from light to dark and viceversa
function switchTheme() {
    var html = document.getElementById("html")
    //console.log("theme = "+html.className)
    if (html.className == "theme_light") {
        //console.log("Setting dark");
        html.className = "theme_dark";
    } else {
        //console.log("Setting light");
        html.className = "theme_light";
    }
}