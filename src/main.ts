import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import './style.css';

// VXE Table imports
import VxeTable from 'vxe-table';
import 'vxe-table/lib/style.css';
import VxeUI from 'vxe-pc-ui';
import 'vxe-pc-ui/lib/style.css';

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(VxeUI);
app.use(VxeTable);
app.mount('#app');
