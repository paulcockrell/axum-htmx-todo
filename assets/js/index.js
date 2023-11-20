import 'htmx.org/dist/htmx.js';
import _hyperscript from 'hyperscript.org';
import '@fortawesome/fontawesome-free/js/all';
import '../css/styles.css';

window.htmx = require('htmx.org/dist/htmx.js')
require('htmx.org/dist/ext/response-targets');
require('htmx.org/dist/ext/loading-states');
_hyperscript.browserInit();

console.log("All systems nominal!");
