

<template>
    <navbar title="首页" showBack=true />
    <el-tabs tab-position="left" class="tabs">
        <el-tab-pane label="User">User111</el-tab-pane>    
        <el-tab-pane label="Config">Config111</el-tab-pane>    
        <el-tab-pane label="Role">Role111</el-tab-pane>    
        <el-tab-pane label="Task">Task111</el-tab-pane> 
    </el-tabs>
    <!-- <el-radio-group :tab-position="left" style="height: 200px;" class="tabs">
        <el-tab-pane label="User">User</el-tab-pane>    
        <el-tab-pane label="Config">Config</el-tab-pane>    
        <el-tab-pane label="Role">Role</el-tab-pane>    
        <el-tab-pane label="Task">Task</el-tab-pane>    
    </el-radio-group> -->
    <el-button @click="testRust()">点击我1</el-button>
    <div class="box" @click="handleClick()">
        <div class="title" @click="handleClick()">
            {{ content }}
        </div>
    </div>
</template>

<script>
import navbar from "../../common/builder/navbar.vue";
import  {  ElRadioGroup, ElTabPane, ElButton, ElTabs } from "element-plus";
import { invoke } from "@tauri-apps/api/core";
export default {
  
    name: "HomePage", 
    components: {
        navbar,
        [ElRadioGroup.name]: ElRadioGroup,
        [ElTabPane.name]: ElTabPane,
        [ElButton.name]: ElButton,
        [ElTabs.name]: ElTabs,
    },
    props: {
        propName: {
            type: Number,
            default: 1
        },
    },
    data() {
        return {
            detailId : "0",
            content : "1231231231231",
            tabs: ['积分', '实物', '卡券'],
        }
    },
    methods: {
        handleClick() {
            this.content = "456456456"
            console.log("handleClick")
            this.$router.push({name:"SecondPage"})
            
        },
        selectedTab(index) {
            console.log("selectedTab", index)
        },
        async testRust() {
            await invoke("greet_action", { name: "123"});
        }
    },
    watch: {
        data(newValue, oldValue) {
            if (newValue.id!== oldValue.id) {
                this.detailId = newValue.id
            }
            
        }
    },
}

</script>

<style lang="less" scoped>
    @import '../css/homepage.less';

</style>
