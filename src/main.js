

import { createApp } from "vue";
import App from './App.vue'
import LaunchPage from './pages/components/launchpage.vue'
import HomePage from './pages/components/homepage.vue'
import SecondPage from './pages/components/secondPage.vue'
import { createMemoryHistory, createRouter} from "vue-router";
import ElementPlus,{ElLoading}  from "element-plus";
import "element-plus/dist/index.css";
import Toast from "./common/js/toast.ts";
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
let app = createApp(App);
let loading;
let loadingCount = 0;
app.config.globalProperties.showLoading = () => {
  if (loadingCount === 0) {
    loading = ElLoading.service({
      lock: true,
      text: "Loading",
      background: "rgba(0, 0, 0, 0.7)"
    });
    loadingCount++;
  }
};
app.config.globalProperties.closeLoading = () => {
  if (loadingCount <= 0) return;
  loadingCount--;
  if (loadingCount === 0) {
    if (loading) {
      loading.close();
    }
  }
};
app.config.globalProperties.$toast = Toast;
app.use(router).use(ElementPlus).mount("#app");
