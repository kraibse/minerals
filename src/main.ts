import { createApp } from "vue";
import App from "./App.vue";

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

import { faTrash, faArrowLeft, faArrowRight, faArrowUp, faQuestion, faFileImage, faFileVideo, faFile, faPlus, faFolder, faFolderOpen, faFolderPlus, faFolderMinus, faFolderTree, faHouse, faChevronRight, faChevronDown } from '@fortawesome/free-solid-svg-icons'

library.add(faTrash, faArrowLeft, faArrowRight, faArrowUp, faQuestion, faFileImage, faFileVideo, faFile, faPlus, faFolder, faFolderOpen, faFolderPlus, faFolderMinus, faFolderTree, faHouse, faChevronRight, faChevronDown)

createApp(App).mount("#app");
