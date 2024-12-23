<!--
  H5的可切换tab栏
  传参:tabs为一个可选tab数组
  回调:selectedTab,回传值data为选中tab的index索引,开发人员根据索引值的不同做相关操作
  author:lite
-->
<template>
    <div class="aresTab"
      :class="setRound?'aresTabShort':''">
      <div v-for="(item,index) of tabs"
        :key="index"
        @click="selectedTab(index)">
        <span :style="select==index?themeColor=='gold'?'color:#D5AE69':'color:#F15F50;':''">{{item}}</span>
        <div :class="themeColor=='gold'?'downDivGold':'downDiv'"
          v-show="select==index"></div>
      </div>
    </div>
</template>
<script>
  export default {
    name: 'aresTab',
    props: {
      tabs: {
        type: Object,
        default: ['tab1', 'tab2', 'tab3', 'tab4', 'tab5', 'tab6', 'tab7']
      },
      select: {
        type: Number,
        default: 0
      },
      themeColor: {
        type: String,
        default: ''
      }
    },
    data() {
      return {
        setRound: {
          type: Boolean,
          default: true
        }
      };
    },
  
    mounted() {
      this.initTabNav();
    },
  
    methods: {
      initTabNav() {
        let clientWidth = document.documentElement.clientWidth;
        if (clientWidth >= this.tabs.length * 52) {
          this.setRound = true;
        } else {
          this.setRound = false;
        }
      },
      selectedTab(data) {
        this.select = data;
        this.$emit('selectedTab', data);
      }
    }
  };
  </script>
  
  <style lang="less" scoped>
  .aresTabShort {
    justify-content: space-around;
  }
  .aresTab {
    width: 100%;
    display: flex;
    // justify-content: space-around;
  
    background-color: #ffffff;
    padding-top: 0.13rem;
    color: #333333;
    font-size: 0.14rem;
    user-select: none;
    overflow-x: scroll;
    overflow-y: hidden;
    -webkit-overflow-scrolling: touch;
    z-index: 999;
    & > div {
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 0 0.03rem;
      & > span {
        white-space: nowrap;
        margin-bottom: 0.13rem;
        text-align: center;
        min-width: 0.52rem;
      }
      & .downDiv {
        height: 0.04rem;
        background-color: #f15f50;
        width: 0.52rem;
      }
      & .downDivGold {
        height: 0.04rem;
        background-image: linear-gradient(135deg, #efd38e 0%, #d0a762 100%);
        width: 0.52rem;
      }
    }
  }
  .aresTab::-webkit-scrollbar {
    display: none;
  }
  </style>
  