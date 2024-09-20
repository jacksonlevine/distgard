use once_cell::sync::Lazy;

use crate::specialblocks::vertexutils::rotate_coordinates_around_y_negative_90;
// use crate::textureface::TextureFace;







pub struct TorchInfo {
    
}


impl TorchInfo {



    pub fn torch_model_from_index(index: usize) -> &'static Vec<f32> {
        static MODELS: Lazy<Vec<Vec<f32>>> = Lazy::new(|| {
            vec![
                TorchInfo::base_torch_model().to_vec(),
                rotate_coordinates_around_y_negative_90(TorchInfo::base_torch_model(), 1),
                rotate_coordinates_around_y_negative_90(TorchInfo::base_torch_model(), 2),
                rotate_coordinates_around_y_negative_90(TorchInfo::base_torch_model(), 3)
            ]
        });
        &(*MODELS)[index]
    }

    pub fn get_torch_uvs() -> Vec<f32> {


        let uvs = vec![
            0.31604233384132385, 0.8673124611377716, 0.0, 0.0,
0.31339797377586365, 0.8476128578186035, 0.0, 0.0,
0.31339797377586365, 0.8673124611377716, 0.0, 0.0,
0.31339797377586365, 0.8673124611377716, 0.0, 0.0,
0.31339797377586365, 0.8476128578186035, 0.0, 0.0,
0.31604233384132385, 0.8673124611377716, 0.0, 0.0,
0.31600937247276306, 0.8671908229589462, 0.0, 0.0,
0.3133767247200012, 0.8475784212350845, 0.0, 0.0,
0.3133767247200012, 0.8671908229589462, 0.0, 0.0,
0.3133767247200012, 0.8671908229589462, 0.0, 0.0,
0.3133767247200012, 0.8475784212350845, 0.0, 0.0,
0.31600937247276306, 0.8671908229589462, 0.0, 0.0,
0.31340456008911133, 0.8672943413257599, 0.0, 0.0,
0.31604713201522827, 0.8475956320762634, 0.0, 0.0,
0.31340542435646057, 0.8476623594760895, 0.0, 0.0,
0.31340542435646057, 0.8476623594760895, 0.0, 0.0,
0.31604713201522827, 0.8475956320762634, 0.0, 0.0,
0.31340456008911133, 0.8672943413257599, 0.0, 0.0,
0.31604936718940735, 0.8673238903284073, 0.0, 0.0,
0.31339898705482483, 0.8475794494152069, 0.0, 0.0,
0.31339898705482483, 0.8673238903284073, 0.0, 0.0,
0.31339898705482483, 0.8673238903284073, 0.0, 0.0,
0.31339898705482483, 0.8475794494152069, 0.0, 0.0,
0.31604936718940735, 0.8673238903284073, 0.0, 0.0,
0.31602686643600464, 0.8671258985996246, 0.0, 0.0,
0.3133997321128845, 0.8475546091794968, 0.0, 0.0,
0.3133997321128845, 0.8671258985996246, 0.0, 0.0,
0.3133997321128845, 0.8671258985996246, 0.0, 0.0,
0.3133997321128845, 0.8475546091794968, 0.0, 0.0,
0.31602686643600464, 0.8671258985996246, 0.0, 0.0,
0.3133983016014099, 0.8673000633716583, 0.0, 0.0,
0.3107541501522064, 0.8476017266511917, 0.0, 0.0,
0.3107541501522064, 0.8673000633716583, 0.0, 0.0,
0.3107541501522064, 0.8673000633716583, 0.0, 0.0,
0.3107541501522064, 0.8476017266511917, 0.0, 0.0,
0.3133983016014099, 0.8673000633716583, 0.0, 0.0,
0.31604233384132385, 0.8673124611377716, 0.0, 0.0,
0.31604233384132385, 0.8476128578186035, 0.0, 0.0,
0.31339797377586365, 0.8476128578186035, 0.0, 0.0,
0.31339797377586365, 0.8476128578186035, 0.0, 0.0,
0.31604233384132385, 0.8476128578186035, 0.0, 0.0,
0.31604233384132385, 0.8673124611377716, 0.0, 0.0,
0.31600937247276306, 0.8671908229589462, 0.0, 0.0,
0.31600937247276306, 0.8475784212350845, 0.0, 0.0,
0.3133767247200012, 0.8475784212350845, 0.0, 0.0,
0.3133767247200012, 0.8475784212350845, 0.0, 0.0,
0.31600937247276306, 0.8475784212350845, 0.0, 0.0,
0.31600937247276306, 0.8671908229589462, 0.0, 0.0,
0.31340456008911133, 0.8672943413257599, 0.0, 0.0,
0.31604626774787903, 0.8673365712165833, 0.0, 0.0,
0.31604713201522827, 0.8475956320762634, 0.0, 0.0,
0.31604713201522827, 0.8475956320762634, 0.0, 0.0,
0.31604626774787903, 0.8673365712165833, 0.0, 0.0,
0.31340456008911133, 0.8672943413257599, 0.0, 0.0,
0.31604936718940735, 0.8673238903284073, 0.0, 0.0,
0.31604936718940735, 0.8475794494152069, 0.0, 0.0,
0.31339898705482483, 0.8475794494152069, 0.0, 0.0,
0.31339898705482483, 0.8475794494152069, 0.0, 0.0,
0.31604936718940735, 0.8475794494152069, 0.0, 0.0,
0.31604936718940735, 0.8673238903284073, 0.0, 0.0,
0.31602686643600464, 0.8671258985996246, 0.0, 0.0,
0.31602686643600464, 0.8475546091794968, 0.0, 0.0,
0.3133997321128845, 0.8475546091794968, 0.0, 0.0,
0.3133997321128845, 0.8475546091794968, 0.0, 0.0,
0.31602686643600464, 0.8475546091794968, 0.0, 0.0,
0.31602686643600464, 0.8671258985996246, 0.0, 0.0,
0.3133983016014099, 0.8673000633716583, 0.0, 0.0,
0.3133983016014099, 0.8476017266511917, 0.0, 0.0,
0.3107541501522064, 0.8476017266511917, 0.0, 0.0,
0.3107541501522064, 0.8476017266511917, 0.0, 0.0,
0.3133983016014099, 0.8476017266511917, 0.0, 0.0,
0.3133983016014099, 0.8673000633716583, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8382279276847839, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8474058210849762, 0.0, 0.0,
0.31799423694610596, 0.8382279276847839, 0.0, 0.0,
0.3088163435459137, 0.8382279276847839, 0.0, 0.0,



        ];
        uvs
    }

    pub fn base_torch_model() -> &'static [f32] {
        static PLAYER_IS_MINUS_Z: [f32; 600] = [0.42930662631988525, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, -0.019913578405976295, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.5733066201210022, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.4119277894496918, 0.0, 14.0,
        0.42930662631988525, 0.5400863885879517, 0.5674477815628052, 0.0, 14.0,
        0.6339267492294312, 0.7678297162055969, 0.5238243341445923, 0.0, 14.0,
        0.3895680606365204, 0.5198028087615967, 0.572140634059906, 0.0, 14.0,
        0.3895680606365204, 0.7678297162055969, 0.5238243341445923, 0.0, 14.0,
        0.3895680606365204, 0.7678297162055969, 0.5238243341445923, 0.0, 14.0,
        0.3895680606365204, 0.5198028087615967, 0.572140634059906, 0.0, 14.0,
        0.6339267492294312, 0.7678297162055969, 0.5238243341445923, 0.0, 14.0,
        0.5425876379013062, 0.7822644710540771, 0.37147146463394165, 0.0, 14.0,
        0.5899770855903625, 0.5342375040054321, 0.6160115003585815, 0.0, 14.0,
        0.5416611433029175, 0.7822644710540771, 0.615828275680542, 0.0, 14.0,
        0.5416611433029175, 0.7822644710540771, 0.615828275680542, 0.0, 14.0,
        0.5899770855903625, 0.5342375040054321, 0.6160115003585815, 0.0, 14.0,
        0.5425876379013062, 0.7822644710540771, 0.37147146463394165, 0.0, 14.0,
        0.6442495584487915, 0.5357531905174255, 0.3959883451461792, 0.0, 14.0,
        0.3998908996582031, 0.7837800979614258, 0.44430458545684814, 0.0, 14.0,
        0.6442495584487915, 0.7837800979614258, 0.4443046450614929, 0.0, 14.0,
        0.6442495584487915, 0.7837800979614258, 0.4443046450614929, 0.0, 14.0,
        0.3998908996582031, 0.7837800979614258, 0.44430458545684814, 0.0, 14.0,
        0.6442495584487915, 0.5357531905174255, 0.3959883451461792, 0.0, 14.0,
        0.44384056329727173, 0.5234615802764893, 0.3521174192428589, 0.0, 14.0,
        0.49122998118400574, 0.7714885473251343, 0.596657395362854, 0.0, 14.0,
        0.4921565055847168, 0.7714885473251343, 0.35230064392089844, 0.0, 14.0,
        0.4921565055847168, 0.7714885473251343, 0.35230064392089844, 0.0, 14.0,
        0.49122998118400574, 0.7714885473251343, 0.596657395362854, 0.0, 14.0,
        0.44384056329727173, 0.5234615802764893, 0.3521174192428589, 0.0, 14.0,
        0.6339267492294312, 0.7678297162055969, 0.5238243341445923, 0.0, 14.0,
        0.6339267492294312, 0.5198028087615967, 0.572140634059906, 0.0, 14.0,
        0.3895680606365204, 0.5198028087615967, 0.572140634059906, 0.0, 14.0,
        0.3895680606365204, 0.5198028087615967, 0.572140634059906, 0.0, 14.0,
        0.6339267492294312, 0.5198028087615967, 0.572140634059906, 0.0, 14.0,
        0.6339267492294312, 0.7678297162055969, 0.5238243341445923, 0.0, 14.0,
        0.5425876379013062, 0.7822644710540771, 0.37147146463394165, 0.0, 14.0,
        0.5909035801887512, 0.5342375040054321, 0.3716546297073364, 0.0, 14.0,
        0.5899770855903625, 0.5342375040054321, 0.6160115003585815, 0.0, 14.0,
        0.5899770855903625, 0.5342375040054321, 0.6160115003585815, 0.0, 14.0,
        0.5909035801887512, 0.5342375040054321, 0.3716546297073364, 0.0, 14.0,
        0.5425876379013062, 0.7822644710540771, 0.37147146463394165, 0.0, 14.0,
        0.6442495584487915, 0.5357531905174255, 0.3959883451461792, 0.0, 14.0,
        0.3998909592628479, 0.5357531905174255, 0.3959882855415344, 0.0, 14.0,
        0.3998908996582031, 0.7837800979614258, 0.44430458545684814, 0.0, 14.0,
        0.3998908996582031, 0.7837800979614258, 0.44430458545684814, 0.0, 14.0,
        0.3998909592628479, 0.5357531905174255, 0.3959882855415344, 0.0, 14.0,
        0.6442495584487915, 0.5357531905174255, 0.3959883451461792, 0.0, 14.0,
        0.44384056329727173, 0.5234615802764893, 0.3521174192428589, 0.0, 14.0,
        0.44291403889656067, 0.5234615802764893, 0.596474289894104, 0.0, 14.0,
        0.49122998118400574, 0.7714885473251343, 0.596657395362854, 0.0, 14.0,
        0.49122998118400574, 0.7714885473251343, 0.596657395362854, 0.0, 14.0,
        0.44291403889656067, 0.5234615802764893, 0.596474289894104, 0.0, 14.0,
        0.44384056329727173, 0.5234615802764893, 0.3521174192428589, 0.0, 14.0,
        
        ];
        &PLAYER_IS_MINUS_Z
    }
}