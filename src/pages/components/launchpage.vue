<template>
  <main class="container">
    <!-- <p class="select-area" @click="showDialog">测试弹窗</p>
    <p class="select-area" @click="showLoading">测试loading</p> -->
    <!-- <p class="select-area" @click="hideLoading">隐藏loading</p> -->
    <p class="select-area" @click="rustSelectIpListFile">点击调用rust选择文件</p>

    <div class="list-area">
      <div class="list-item" @click="showDialog(item.ip)" v-for="(item, index) in check_list" :key="index">
        <span class="text">{{ item.ip }}</span>
        <div :class="status_Css(item.status)"></div>

      </div>
    </div>



  </main>
</template>

<script>
import { invoke } from "@tauri-apps/api/core";
import { ElLoading, ElMessage } from 'element-plus'
import { withLoading,showLoading,hideLoading } from "../../common/js/loading";
export default {
  name: 'LaunchPage',
  components: {
    ElLoading,
    ElMessage
  },
  data() {
    return {
      greetMsg: "",
      inputMsg: "185.199.108.134",
      ipList: [],
      check_list: [
      ],

      fileLocalPath: "拖拽文件到此处",
      rust_local_file_path: ""
    }
  },
  mounted() {
    console.log("mounted初始化了");
  },
  computed: {
    status_Css() {
      return (status) => {
        if (status) {
          return "status-on";
        } else {
          return "status-off";
        }
      }
    }
  },
  destroyed() {
  },
  methods: {
    async greet() {
      invoke("greet_action", {});
    },
    async rustSelectFile() {
      invoke("log", { content: "该选择文件啦" });
      this.rust_local_file_path = await invoke("select_local_file", {});
      //  invoke("log", {content: this.rust_local_file_path});
      invoke("install_harmony_package", { ip_address: this.rust_local_file_path, });
    },


    async rustSelectIpListFile() {
      const loading = ElLoading.service({
        lock: true,
        text: '处理中',
        background: 'rgba(0, 0, 0, 0.7)',
      })
      let ips = await invoke("select_local_ip_list", {});
      this.ipList = ips.split("\n");
      this.check_list = [];
      for (var i = 0; i < this.ipList.length; i++) {
        let ip = this.ipList[i];
        this.check_list.push({
          ip: ip,
          status: false
        });
      }
      console.log(this.check_list.length);
      invoke("log", { content: "该选择文件啦" });

      let map = new Object();
      map = await invoke("ip_list_reachable", { list: this.ipList });
      invoke("log", { content: "列表返回数据啦" })
      loading.close()
      for (var i = 0; i < this.ipList.length; i++) {

        let ip = this.ipList[i];
        let result = map[ip];

        for (var j = 0; j < this.check_list.length; j++) {
          let item = this.check_list[j];
          if (item.ip == ip) {
            item.status = result;
          }
        }
      }
    },

    // async selectIpListAndGetRechable() : Promise<String> {
    //   return new Promise(async (resolve) => {
    //     let ips = await invoke("select_local_ip_list", {});
    //     this.ipList = ips.split("\n");
    //     this.check_list = [];
    //     for (var i = 0; i < this.ipList.length; i++) {
    //       let ip = this.ipList[i];
    //       this.check_list.push({
    //         ip: ip,
    //         status: false
    //       });
    //     }
    //     console.log(this.check_list.length);
    //     invoke("log", { content: "该选择文件啦" });

    //     let map = new Object();
    //     map = await invoke("ip_list_reachable", { list: this.ipList });
    //   resolve(map);
    //   });
    // },


    dropActions(e) {
      e.preventDefault();
      if (e.type === "dragover") {
        e.dataTransfer.dropEffect = "copy";
      } else if (e.type === "drop") {
        e.stopPropagation();
        var files = e.dataTransfer.files;
        this.fileLocalPath = files[0].name;
      }
    },
    selectFile(e) {
      const file = e.target.files[0];
      invoke("log", { content: file.name });
      invoke("log", { content: file.webkitRelativePath });
    },
    showDialog() {
      this.$toast({
        value: "弹窗测试",
      })
    },
    
    async showLoading() {
      // const loading = ElLoading.service({
      //   lock: true,
      //   text: 'Loading',
      //   background: 'rgba(0, 0, 0, 0.7)',
      // })
      // setTimeout(() => {
      //   loading.close()
      // }, 2000)
      // this.fullscreenLoading = true
      // setTimeout(() => {
      //   this.fullscreenLoading = false
      // }, 2000)
      // this.showLoading();
      // setTimeout(() => {
      //   this.hideLoading();
      // }, 2000)
      //  withLoading(this.asyncDemo()).then((result) => {
      //   console.log(result);
      //   ElMessage.success("成功啦")
      // })
    //  let promise = this.asyncDemo()
    //  promise.then((result) => {
    //     console.log(result);
    //     ElMessage.success("成功啦")
    //   })
      // .then((result) => {
      //   console.log(result);
      //   ElMessage.success("成功啦11")
      // })
      showLoading();
      setTimeout(() => {
        hideLoading();
      }, 2000);
      
    },
    hideLoading() {
      // ElLoading.service.close()
      // this.$loading.hide();
    },
    asyncDemo() {
       let promise = new Promise((resolve, reject) => {
        setTimeout(() => {
          resolve("成功啦")
        }, 2000)
      })
      return promise
        
    },
    demo() {
      this.ipList = [
        "1.1.1.1",
        "192.168.1.1",
      ];
    }
  },
}
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #292929;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1,
p {
  text-align: center;
  color: white;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #292929;
  background-color: white;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

.drop_area {
  align-items: center;
  justify-content: center;
  text-align: center;
  display: flex;
}

.drop_area .tips {
  color: #333333;
  font-size: 14px;
  text-align: center;
  background-color: #292929;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 200px;
  height: 200px;
}

.select-area {
  cursor: pointer;
  color: white;
  font-size: 24px;
  margin-top: 20px;
}


.list-area {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100%;
  /* height: 400px; */
  height: 50%;
  overflow: scroll;
  /* background-color: green; */
}

.list-area .list-item {
  display: flex;
  flex-direction: row;
  /* background-color: pink; */
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  padding: 10px;
  border-radius: 10px;

}

.list-area .list-item .text {
  font-size: 16px;
  font-weight: 500;
  width: 150px;
  color: white;
}

.list-area .list-item .status-on {
  background-color: green;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  margin-left: 10px;
}

.list-area .list-item .status-off {
  background-color: red;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  margin-left: 10px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>