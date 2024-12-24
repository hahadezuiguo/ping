<template>
    <navbar :title="'Second Page'" :showBack="true" />
    <div class="box" @click="handleClick()">
        <div class="title" @click="handleClick()">
            {{ content }}
        </div>
    </div>
</template>

<script>
import navbar from '../../common/builder/navbar.vue';
export default {
    components: {
        navbar
    },
    name: "SecondPage",
    props: {
        propName: {
            type: Number,
            default: 1
        },
    },
    data() {
        return {
            detailId: "0",
            content: "1231231231231"
        }
    },
    methods: {
        handleClick() {
            this.content = "456456456"
            console.log("handleClick")

        },
        showWindow() {
            const testWindow = WebviewWindow.get("testWindow");
        }
    },
    watch: {
        data(newValue, oldValue) {
            if (newValue.id !== oldValue.id) {
                this.detailId = newValue.id
            }

        }
    },
}

</script>

<style scoped>
.box {
    background-color: green;
    padding: 20px;
}

.box .title {
    background-color: green;
    font-size: 18px;
    color: blue;
}
</style>
