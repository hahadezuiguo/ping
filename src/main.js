

import { createApp } from "vue";
import App from './App.vue'
import LaunchPage from './pages/components/launchpage.vue'
import HomePage from './pages/components/homepage.vue'
import SecondPage from './pages/components/secondPage.vue'
import { createMemoryHistory, createRouter} from "vue-router";
import ElementPlus  from "element-plus";
import "element-plus/dist/index.css";
// import toast from './common/js/toast.js'
// import ToastComponent from './common/builder/toast.vue'
const router = new createRouter({
    history: createMemoryHistory(),
    routes: [
      {
        path: '/',
        name: 'LaunchPage',
        component: LaunchPage
      },
      {
        path: "/HomePage",
        name: "HomePage",
        component: HomePage
      },
      {
        path: "/SecondPage",
        name: "SecondPage",
        component: SecondPage
      }
    ]
  })




// createApp({
//   el: '#app',
//   router,
//   components: { LaunchPage },
//   template: '<App/>'
// })
createApp(App).use(router).use(ElementPlus).mount("#app");
