import ToastComponent from '../builder/toast.vue'    //引入toast.vue组件

export default {    // 导出一个对象，里面包含vue注册插件所调用的方法install
    install: function (Vue) {
        const ToastConstructor = Vue.extend(ToastComponent)    // 将toast.vue组件生成为Vue的子类
        const instance = new ToastConstructor();    // 生成一个该子类的实例

        instance.$mount(document.createElement('div'))    // 将这个实例挂载在新创建的div上，并加入到body中
        document.body.appendChild(instance.$el)

        // 通过Vue的原型注册一个方法$toast，有两个参数（msg为提示的文字，duration为延时关闭）
        Vue.prototype.$toast = (msg, duration = 1500) => {
            if (instance.visible) return;    // visible是toast.vue组件的一个变量，用来控制提示框的显示

            instance.message = msg;
            instance.visible = true;

            setTimeout(() => {    // 默认延时1.5s关闭提示框
                instance.visible = false;
            }, duration);
        }
    }
}