import { createApp } from "vue";
import App from "./App.vue";

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

import { faFolder, faFolderOpen, faFolderPlus, faFolderMinus, faFolderTree } from '@fortawesome/free-solid-svg-icons'

library.add(faFolder, faFolderOpen, faFolderPlus, faFolderMinus, faFolderTree)

createApp(App).mount("#app");
