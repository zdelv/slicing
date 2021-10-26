import * as THREE from 'three';
import {slice_model, load_model, get_centroid} from "../pkg/index_bg.js";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import {VertexNormalsHelper} from "three/examples/jsm/helpers/VertexNormalsHelper";

const scene = new THREE.Scene();
const canvas = document.querySelector("#main-canvas");
const camera = new THREE.PerspectiveCamera( 75, canvas.clientWidth / canvas.clientHeight, 0.001, 1000 );
// const aspect = canvas.clientWidth / canvas.clientHeight;
// const camera = new THREE.OrthographicCamera(-1,1,1,-1, 0.001, 1000);

const renderer = new THREE.WebGLRenderer({ canvas: document.querySelector("#main-canvas"), antialias: true });
renderer.setSize( renderer.domElement.innerWidth, renderer.domElement.innerHeight );

const light = new THREE.HemisphereLight(0xffffff, 0xffffff, 0.25);
scene.add(light);
const dir = new THREE.DirectionalLight(0xffffff, 1.5);
//dir.position.set(-1, 1, 1);
camera.add(dir);
scene.add(camera);

const controls = new OrbitControls(camera, renderer.domElement);
camera.position.set(1,1,1);
camera.lookAt(0,0,0);
controls.update()

function resizeCanvas() {
    const canvas = renderer.domElement;
    const width = canvas.clientWidth;
    const height = canvas.clientHeight;

    if (canvas.width !== width || canvas.height !== height) {
        renderer.setSize(width, height, false);
        renderer.setPixelRatio(window.devicePixelRatio);
        camera.aspect = width / height;
        camera.updateProjectionMatrix();
    }
}

function onDrop(event) {
    console.log("File(s) dropped.");

    event.preventDefault();

    if (event.dataTransfer.items) {
        // Use DataTransferItemList interface
        for (let i = 0; i < event.dataTransfer.items.length; i++) {
            if (event.dataTransfer.items[i].kind === "file") {
                const file = event.dataTransfer.items[i].getAsFile();
                console.log("... file[" + i + "].name = " + file.name);
                file.arrayBuffer().then(data => {
                    dropped_data = new Uint8Array(data);
                    load_data()
                })
            }
        }
    } else {
        // Use DataTransfer interface
        for (let i = 0; i < event.dataTransfer.files.length; i++) {
            console.log("... file[" + i + "].name = " + event.dataTransfer.files[i].name);
        }
    }
}

function load_data() {
    if (!(dropped_data.length > 0)) {
        console.error("No data has been loaded!");
        return;
    }

    model = JSON.parse(load_model(dropped_data));
    document.querySelector("#model_name_field").textContent = model["name"]
    document.querySelector("#vertex_count_field").textContent = model["verts"].length
    document.querySelector("#triangle_count_field").textContent = model["tris"].length

    update_model_display();
}

function update_model_display() {
    if (!(dropped_data.length > 0)) {
        console.error("No data has been loaded!");
        return;
    }

    const geometry = new THREE.BufferGeometry();

    const vertices = ((data) => {
        let out = [];
        for (let vert of data["verts"]) {
            out.push(vert.x);
            out.push(vert.z);
            out.push(vert.y);
        }
        return out
    })(model);

    const indices = ((data) => {
        let out = [];
        for (let tri of data["tris"]) {
            out.push(tri["v1"]);
            out.push(tri["v2"]);
            out.push(tri["v3"]);
        }
        return out
    })(model);

    console.log(vertices);
    console.log(indices);

    geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
    geometry.setIndex(indices);
    geometry.computeVertexNormals();

    const material = new THREE.MeshStandardMaterial({color: 0x555555, side: THREE.DoubleSide, metallicness: 0.5})
    material.flatShading = true;
    const mesh = new THREE.Mesh(geometry, material);

    let out = JSON.parse(get_centroid(dropped_data));
    controls.target.set(out.x, out.z, out.y)

    scene.add(mesh);
    // const helper = new VertexNormalsHelper(mesh);
    // scene.add(helper);

    // const wireframe = new THREE.WireframeGeometry(geometry);
    // const line = new THREE.LineSegments(wireframe);
    // scene.add(line);
}

function slice_data() {
    if (model && Object.keys(model).length === 0 && Object.getPrototypeOf(model) === Object.prototype) {
        console.error("Model has not been loaded!");
    }

    let out = slice_model(dropped_data);
    slices = JSON.parse(out);

    document.querySelector("#slice_count_field").textContent = slices.length

    const geo = new THREE.PlaneGeometry(1,1);
    const material = new THREE.MeshStandardMaterial({color: 0x1133ff, side: THREE.DoubleSide, opacity: 0.5, transparent: true});

    let ids = [];
    for (let slice of slices) {
        const plane = new THREE.Mesh(geo, material);
        scene.add(plane);

        const normal = new THREE.Vector3(slice.normal.x, slice.normal.z, slice.normal.y);
        plane.lookAt(normal);
        plane.position.set(slice.point.x, slice.point.z, slice.point.y);
        plane.translateOnAxis(normal, -0.001);
        ids.push(plane.id);
    }

    add_slices_to_select(ids)

}

function add_slices_to_select(ids) {
    let select = document.querySelector("#slice_list_select")

    for (let i = 0; i < slices.length; i++) {
        let opt = document.createElement("option");
        opt.value = ids[i];
        opt.innerHTML = "Slice " + i.toString();
        select.appendChild(opt)
    }
}

document.querySelector("#slice_list_select").addEventListener('click', (e) => {
    let select = document.querySelector("#slice_list_select");
    let selected_options = Array.from(select.selectedOptions);
    let options = select.children;
    for (let option of options) {
        let obj = scene.getObjectById(parseInt(option.value));
        if (selected_options.includes(option)) {
            obj.visible = true;
        } else {
            obj.visible = false;
        }
    }
})


let dropped_data = new Uint8Array(0);
let model = {};
let slices = {};

document.querySelector("#load_data_button").onclick = load_data
document.querySelector("#slice_model_button").onclick = slice_data


;['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
    document.body.addEventListener(eventName, preventDefaults, false);
})

document.body.addEventListener('drop', onDrop, false)

function preventDefaults(e) {
    e.preventDefault();
    e.stopPropagation();
}

function animate(time) {
    resizeCanvas()
    controls.update()

    renderer.render( scene, camera );
    window.requestAnimationFrame( animate );
}
animate()
