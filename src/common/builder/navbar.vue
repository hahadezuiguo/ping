<template>
    <div class="navbar">
        <img v-if="showBack" class="icon" src="" @click="back()"/>
        <div class="title">{{ title }}</div>
        <div></div>
    </div>
</template>

<script>

    export default {
        // name: 'Navbar',s
        props: {
            title: "默认标题",
            showBack: true,
        },
        methods: {
            back() {
                this.$router.back();
            }
        },
    }
</script>

<style lang="less" scoped>
    @import "../css/navbar.less";
</style>