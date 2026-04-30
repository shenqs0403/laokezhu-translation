import {ref} from "vue";
import {Engine} from "./Common.ts";

// 基础配置信息对象
export const basic = ref({shortcut: "", swipe: 300})
// 当前编辑的引擎
export const currentEngine = ref<Engine>({
    engine_key: "",
    engine_name: "",
    engine_zh_name: "",
    url: "",
    appid: "",
    enable: false,
    region: ""
});

export const TENCENT_REGIONS =[
    { label: "亚太东南（曼谷）", value: "ap-bangkok" },
    { label: "华北地区（北京）", value: "ap-beijing" },
    { label: "西南地区（成都）", value: "ap-chengdu" },
    { label: "西南地区（重庆）", value: "ap-chongqing" },
    { label: "华南地区（广州）", value: "ap-guangzhou" },
    { label: "港澳台地区（中国香港）", value: "ap-hongkong" },
    { label: "亚太东北（首尔）", value: "ap-seoul" },
    { label: "华东地区（上海）", value: "ap-shanghai" },
    { label: "华东地区（上海金融）", value: "ap-shanghai-fsi" },
    { label: "华南地区（深圳金融）", value: "ap-shenzhen-fsi" },
    { label: "亚太东南（新加坡）", value: "ap-singapore" },
    { label: "亚太东北（东京）", value: "ap-tokyo" },
    { label: "欧洲地区（法兰克福）", value: "eu-frankfurt" },
    { label: "美国东部（弗吉尼亚）", value: "na-ashburn" },
    { label: "美国西部（硅谷）", value: "na-siliconvalley" }
];