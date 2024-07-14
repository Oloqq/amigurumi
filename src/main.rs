mod args;
mod common;
mod comparison;
mod flow;
mod plushie;
mod rocket_server;
mod ws_sim;
extern crate nalgebra as na;
#[macro_use]
extern crate rocket;

use flow::actions::Action;
use flow::ergoflow::ErgoFlow;

use self::args::*;
use self::ws_sim::plushie_sim::PlushieSimulation;
use crate::flow::pest_parser::Pattern;
use crate::plushie::examples;
use crate::plushie::Params;
use crate::plushie::{Plushie, Pointcloud};
use crate::ws_sim::serve_websocket;
use std::fs;
use std::io::Write;

fn main() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let args = Args::from_args();
    use Command::*;
    match args.cmd {
        WebSocket(args) => {
            let plushie = examples::ergogrzib();
            let sim = PlushieSimulation::from(plushie);
            serve_websocket(sim, format!("127.0.0.1:{}", args.port).as_str());
        }
        Dev { num } => {
            match num {
                1 => {
                    let d = Params::default();
                    let s = serde_json::to_string_pretty(&d).unwrap();
                    println!("{s}");
                }
                2 => {
                    let plushie = examples::ergogrzib();
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                3 => {
                    let plushie = Pointcloud::from_stl("models/grzib40.stl");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                4 => {
                    let plushie = Pointcloud::from_points_file("models/grzib10cloud1000.json");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                5 => {
                    // const SHAPE_INIT: &str = "[[0.31830987334251404,0.0,0.0],[0.19846296310424805,0.0,0.24886468052864075],[-0.07083061337471008,0.0,0.3103291690349579],[-0.2867873013019562,0.0,0.13810943067073822],[-0.2867872714996338,0.0,-0.138109490275383],[-0.07083063572645187,0.0,-0.3103291690349579],[0.19846303761005402,0.0,-0.24886463582515717],[0.517253577709198,0.699999988079071,0.0],[0.4479547441005707,0.699999988079071,0.258626788854599],[0.2586267590522766,0.699999988079071,0.44795477390289307],[-2.2609871308532092e-8,0.699999988079071,0.517253577709198],[-0.2586268186569214,0.699999988079071,0.4479547441005707],[-0.4479547441005707,0.699999988079071,0.2586268186569214],[-0.517253577709198,0.699999988079071,-4.5219742617064185e-8],[-0.4479546546936035,0.699999988079071,-0.25862687826156616],[-0.2586267292499542,0.699999988079071,-0.44795477390289307],[6.168187294264271e-9,0.699999988079071,-0.517253577709198],[0.2586267292499542,0.699999988079071,-0.44795477390289307],[0.44795483350753784,0.699999988079071,-0.25862666964530945],[0.517253577709198,1.399999976158142,0.0],[0.4479547441005707,1.399999976158142,0.258626788854599],[0.2586267590522766,1.399999976158142,0.44795477390289307],[-2.2609871308532092e-8,1.399999976158142,0.517253577709198],[-0.2586268186569214,1.399999976158142,0.4479547441005707],[-0.4479547441005707,1.399999976158142,0.2586268186569214],[-0.517253577709198,1.399999976158142,-4.5219742617064185e-8],[-0.4479546546936035,1.399999976158142,-0.25862687826156616],[-0.2586267292499542,1.399999976158142,-0.44795477390289307],[6.168187294264271e-9,1.399999976158142,-0.517253577709198],[0.2586267292499542,1.399999976158142,-0.44795477390289307],[0.44795483350753784,1.399999976158142,-0.25862666964530945],[0.517253577709198,2.0999999046325684,0.0],[0.4479547441005707,2.0999999046325684,0.258626788854599],[0.2586267590522766,2.0999999046325684,0.44795477390289307],[-2.2609871308532092e-8,2.0999999046325684,0.517253577709198],[-0.2586268186569214,2.0999999046325684,0.4479547441005707],[-0.4479547441005707,2.0999999046325684,0.2586268186569214],[-0.517253577709198,2.0999999046325684,-4.5219742617064185e-8],[-0.4479546546936035,2.0999999046325684,-0.25862687826156616],[-0.2586267292499542,2.0999999046325684,-0.44795477390289307],[6.168187294264271e-9,2.0999999046325684,-0.517253577709198],[0.2586267292499542,2.0999999046325684,-0.44795477390289307],[0.44795483350753784,2.0999999046325684,-0.25862666964530945],[0.517253577709198,2.799999952316284,0.0],[0.4479547441005707,2.799999952316284,0.258626788854599],[0.2586267590522766,2.799999952316284,0.44795477390289307],[-2.2609871308532092e-8,2.799999952316284,0.517253577709198],[-0.2586268186569214,2.799999952316284,0.4479547441005707],[-0.4479547441005707,2.799999952316284,0.2586268186569214],[-0.517253577709198,2.799999952316284,-4.5219742617064185e-8],[-0.4479546546936035,2.799999952316284,-0.25862687826156616],[-0.2586267292499542,2.799999952316284,-0.44795477390289307],[6.168187294264271e-9,2.799999952316284,-0.517253577709198],[0.2586267292499542,2.799999952316284,-0.44795477390289307],[0.44795483350753784,2.799999952316284,-0.25862666964530945],[0.2785211503505707,3.5,0.0],[0.13926056027412415,3.5,0.24120640754699707],[-0.13926059007644653,3.5,0.24120639264583588],[-0.2785211503505707,3.5,-2.434909163184784e-8],[-0.13926054537296295,3.5,-0.24120640754699707],[0.13926054537296295,3.5,-0.24120640754699707],[0.07957746833562851,4.199999809265137,0.0],[0.9947183728218079,4.899999618530273,0.0],[0.9608241319656372,4.899999618530273,0.2574520707130432],[0.8614513874053955,4.899999618530273,0.49735918641090393],[0.7033721208572388,4.899999618530273,0.7033721208572388],[0.49735915660858154,4.899999618530273,0.8614514470100403],[0.2574521005153656,4.899999618530273,0.9608241319656372],[-4.3480522293748436e-8,4.899999618530273,0.9947183728218079],[-0.25745218992233276,4.899999618530273,0.9608241319656372],[-0.4973592460155487,4.899999618530273,0.8614513874053955],[-0.7033721208572388,4.899999618530273,0.7033721208572388],[-0.8614513874053955,4.899999618530273,0.4973592460155487],[-0.960824191570282,4.899999618530273,0.25745195150375366],[-0.9947183728218079,4.899999618530273,-8.696104458749687e-8],[-0.9608241319656372,4.899999618530273,-0.2574521005153656],[-0.861451268196106,4.899999618530273,-0.49735936522483826],[-0.7033720016479492,4.899999618530273,-0.7033722400665283],[-0.49735909700393677,4.899999618530273,-0.8614514470100403],[-0.25745201110839844,4.899999618530273,-0.960824191570282],[1.186189813040528e-8,4.899999618530273,-0.9947183728218079],[0.2574520409107208,4.899999618530273,-0.9608241319656372],[0.49735909700393677,4.899999618530273,-0.8614514470100403],[0.7033723592758179,4.899999618530273,-0.7033718824386597],[0.8614515662193298,4.899999618530273,-0.49735894799232483],[0.960824191570282,4.899999618530273,-0.2574518620967865],[0.9947183728218079,5.59999942779541,0.0],[0.9608241319656372,5.59999942779541,0.2574520707130432],[0.8614513874053955,5.59999942779541,0.49735918641090393],[0.7033721208572388,5.59999942779541,0.7033721208572388],[0.49735915660858154,5.59999942779541,0.8614514470100403],[0.2574521005153656,5.59999942779541,0.9608241319656372],[-4.3480522293748436e-8,5.59999942779541,0.9947183728218079],[-0.25745218992233276,5.59999942779541,0.9608241319656372],[-0.4973592460155487,5.59999942779541,0.8614513874053955],[-0.7033721208572388,5.59999942779541,0.7033721208572388],[-0.8614513874053955,5.59999942779541,0.4973592460155487],[-0.960824191570282,5.59999942779541,0.25745195150375366],[-0.9947183728218079,5.59999942779541,-8.696104458749687e-8],[-0.9608241319656372,5.59999942779541,-0.2574521005153656],[-0.861451268196106,5.59999942779541,-0.49735936522483826],[-0.7033720016479492,5.59999942779541,-0.7033722400665283],[-0.49735909700393677,5.59999942779541,-0.8614514470100403],[-0.25745201110839844,5.59999942779541,-0.960824191570282],[1.186189813040528e-8,5.59999942779541,-0.9947183728218079],[0.2574520409107208,5.59999942779541,-0.9608241319656372],[0.49735909700393677,5.59999942779541,-0.8614514470100403],[0.7033723592758179,5.59999942779541,-0.7033718824386597],[0.8614515662193298,5.59999942779541,-0.49735894799232483],[0.960824191570282,5.59999942779541,-0.2574518620967865],[0.9947183728218079,6.299999237060547,0.0],[0.9608241319656372,6.299999237060547,0.2574520707130432],[0.8614513874053955,6.299999237060547,0.49735918641090393],[0.7033721208572388,6.299999237060547,0.7033721208572388],[0.49735915660858154,6.299999237060547,0.8614514470100403],[0.2574521005153656,6.299999237060547,0.9608241319656372],[-4.3480522293748436e-8,6.299999237060547,0.9947183728218079],[-0.25745218992233276,6.299999237060547,0.9608241319656372],[-0.4973592460155487,6.299999237060547,0.8614513874053955],[-0.7033721208572388,6.299999237060547,0.7033721208572388],[-0.8614513874053955,6.299999237060547,0.4973592460155487],[-0.960824191570282,6.299999237060547,0.25745195150375366],[-0.9947183728218079,6.299999237060547,-8.696104458749687e-8],[-0.9608241319656372,6.299999237060547,-0.2574521005153656],[-0.861451268196106,6.299999237060547,-0.49735936522483826],[-0.7033720016479492,6.299999237060547,-0.7033722400665283],[-0.49735909700393677,6.299999237060547,-0.8614514470100403],[-0.25745201110839844,6.299999237060547,-0.960824191570282],[1.186189813040528e-8,6.299999237060547,-0.9947183728218079],[0.2574520409107208,6.299999237060547,-0.9608241319656372],[0.49735909700393677,6.299999237060547,-0.8614514470100403],[0.7033723592758179,6.299999237060547,-0.7033718824386597],[0.8614515662193298,6.299999237060547,-0.49735894799232483],[0.960824191570282,6.299999237060547,-0.2574518620967865],[0.517253577709198,6.999999046325684,0.0],[0.4479547441005707,6.999999046325684,0.258626788854599],[0.2586267590522766,6.999999046325684,0.44795477390289307],[-2.2609871308532092e-8,6.999999046325684,0.517253577709198],[-0.2586268186569214,6.999999046325684,0.4479547441005707],[-0.4479547441005707,6.999999046325684,0.2586268186569214],[-0.517253577709198,6.999999046325684,-4.5219742617064185e-8],[-0.4479546546936035,6.999999046325684,-0.25862687826156616],[-0.2586267292499542,6.999999046325684,-0.44795477390289307],[6.168187294264271e-9,6.999999046325684,-0.517253577709198],[0.2586267292499542,6.999999046325684,-0.44795477390289307],[0.44795483350753784,6.999999046325684,-0.25862666964530945],[0.2785211503505707,7.69999885559082,0.0],[0.13926056027412415,7.69999885559082,0.24120640754699707],[-0.13926059007644653,7.69999885559082,0.24120639264583588],[-0.2785211503505707,7.69999885559082,-2.434909163184784e-8],[-0.13926054537296295,7.69999885559082,-0.24120640754699707],[0.13926054537296295,7.69999885559082,-0.24120640754699707],[0.07957746833562851,8.399998664855957,0.0]]";
                    // const SHAPE_MIDDLE: &str = "[[0.31830987334251404,0.0,0.0],[0.8865070343017578,0.256396621465683,0.7808902859687805],[-0.025073522701859474,0.17737162113189697,1.267836093902588],[-0.8304203152656555,0.1619986593723297,0.5993826389312744],[-0.8772473335266113,0.15010209381580353,-0.4766751527786255],[-0.1826338768005371,0.1101054698228836,-1.2254254817962646],[0.7954577803611755,0.14878563582897186,-0.9089205265045166],[1.3278870582580566,0.7718003392219543,-0.42121621966362],[1.7894681692123413,0.9857795238494873,0.47349971532821655],[1.0859360694885254,0.9095100164413452,1.4912883043289185],[0.15750589966773987,1.1322911977767944,2.0494325160980225],[-0.8830199837684631,0.8823478817939758,1.676591157913208],[-1.6870485544204712,1.1183522939682007,1.0619550943374634],[-1.8692643642425537,0.8847389221191406,0.01785973645746708],[-1.6902639865875244,1.083932638168335,-1.0030980110168457],[-0.8454240560531616,1.0130045413970947,-1.7558234930038452],[0.33475324511528015,1.0322531461715698,-1.8805142641067505],[1.370270013809204,0.9692122936248779,-1.2137717008590698],[1.5845355987548828,1.0062865018844604,-0.14806291460990906],[1.8136963844299316,2.0874032974243164,-0.7260788679122925],[2.0324511528015137,2.354671001434326,0.5563647747039795],[1.3595024347305298,2.2896111011505127,1.7337785959243774],[0.17241397500038147,2.4721133708953857,2.2770814895629883],[-1.0579454898834229,2.2463438510894775,2.010172128677368],[-1.9608418941497803,2.4816877841949463,1.1925240755081177],[-2.2557754516601562,2.249939203262329,0.010350403375923634],[-1.9243862628936768,2.4546749591827393,-1.1712086200714111],[-0.9652319550514221,2.3395798206329346,-2.0164332389831543],[0.3575489819049835,2.3888351917266846,-2.1447699069976807],[1.5304409265518188,2.288508415222168,-1.4654102325439453],[1.9781098365783691,2.4017982482910156,-0.21773110330104828],[1.877709984779358,3.5456442832946777,-0.6628684401512146],[1.953275442123413,3.795640468597412,0.6053320169448853],[1.3183146715164185,3.7330620288848877,1.7715812921524048],[0.14042781293392181,3.8869025707244873,2.185358762741089],[-1.10326087474823,3.693286418914795,2.0068697929382324],[-1.8845901489257812,3.9026403427124023,1.1410104036331177],[-2.278641939163208,3.6975269317626953,-0.027280045673251152],[-1.8521134853363037,3.884862184524536,-1.1342915296554565],[-0.9244646430015564,3.7532105445861816,-2.0281763076782227],[0.4012324810028076,3.821707010269165,-2.084890127182007],[1.5719616413116455,3.695112466812134,-1.4012575149536133],[1.9090676307678223,3.8452625274658203,-0.13383843004703522],[2.181570529937744,4.925231456756592,-0.27339646220207214],[1.8785079717636108,5.220424175262451,0.8885613083839417],[1.0881774425506592,5.125081539154053,2.11033034324646],[0.018992889672517776,5.327974796295166,2.1915876865386963],[-1.3887819051742554,5.12846565246582,1.9709787368774414],[-1.9173508882522583,5.345864295959473,1.0973135232925415],[-2.3909857273101807,5.140700817108154,-0.24804003536701202],[-1.8396788835525513,5.336617469787598,-1.1870639324188232],[-0.8200297951698303,5.1785688400268555,-2.2632713317871094],[0.5757365226745605,5.272756576538086,-2.157015562057495],[1.8859453201293945,5.101150989532471,-1.3925508260726929],[1.7367902994155884,5.292972087860107,-0.14719057083129883],[0.7418668270111084,4.9203925132751465,-0.36942970752716064],[0.7254990339279175,5.059750556945801,0.6707533597946167],[-0.24913974106311798,5.085170745849609,0.9687495827674866],[-0.9526183605194092,5.099052429199219,0.22670738399028778],[-0.3762093186378479,5.131775856018066,-0.8649603128433228],[0.9256650805473328,5.208402633666992,-1.5323566198349],[0.240695059299469,5.43812370300293,-0.2625153064727783],[3.1897501945495605,5.51426887512207,-0.33182165026664734],[3.5110256671905518,5.624311447143555,0.28176650404930115],[3.242757558822632,5.874581813812256,1.4132585525512695],[2.629939079284668,5.8580322265625,2.171719551086426],[1.5727919340133667,5.903700828552246,3.2470147609710693],[0.6826382875442505,5.518767356872559,3.5854439735412598],[0.08036408573389053,6.196451187133789,3.520207643508911],[-0.8182207942008972,5.86152458190918,3.4374208450317383],[-2.1877214908599854,5.985337257385254,2.8686861991882324],[-2.875199556350708,5.528595924377441,2.308532476425171],[-3.029266119003296,6.222832679748535,1.810080885887146],[-3.408628463745117,5.878036975860596,0.9771503210067749],[-3.561206340789795,6.005462646484375,-0.49430668354034424],[-3.4119362831115723,5.54297399520874,-1.3770519495010376],[-3.013744592666626,6.214231491088867,-1.7904772758483887],[-2.489199161529541,5.869914531707764,-2.527904748916626],[-1.2801684141159058,6.014100074768066,-3.3794190883636475],[-0.41679254174232483,5.578520774841309,-3.716315746307373],[0.6145666837692261,6.116311073303223,-3.4733994007110596],[1.686841368675232,5.850133419036865,-3.096268653869629],[2.7889456748962402,5.923369884490967,-1.9844839572906494],[3.0022661685943604,5.818745136260986,-0.6022312045097351],[2.1717610359191895,6.084017276763916,0.7246842980384827],[1.6051603555679321,6.160947322845459,1.2270091772079468],[2.1934926509857178,6.580411911010742,-0.07247047126293182],[2.918689489364624,7.016285419464111,-0.03258141130208969],[2.6873950958251953,7.12578821182251,0.9623016119003296],[2.1770622730255127,7.206609725952148,1.8217459917068481],[1.3422770500183105,7.1290106773376465,2.5019469261169434],[0.6058242917060852,6.817382335662842,2.841703176498413],[0.05408577248454094,7.388005256652832,2.720818281173706],[-0.7315188646316528,7.077544212341309,2.71484637260437],[-1.5723598003387451,7.1761040687561035,2.302884817123413],[-2.1852333545684814,6.775730609893799,1.9194787740707397],[-2.291799783706665,7.383888244628906,1.4106429815292358],[-2.69034743309021,7.070560932159424,0.7215631604194641],[-2.7429068088531494,7.173725128173828,-0.21565546095371246],[-2.7251057624816895,6.768862247467041,-0.9402173757553101],[-2.328840970993042,7.370641231536865,-1.2838115692138672],[-1.9412685632705688,7.051342487335205,-1.9731446504592896],[-1.1592442989349365,7.1731061935424805,-2.4986989498138428],[-0.5311144590377808,6.753429889678955,-2.8382341861724854],[0.09543180465698242,7.266533374786377,-2.712068557739258],[0.9427549242973328,7.026585578918457,-2.6559231281280518],[1.8728028535842896,7.081387996673584,-2.0755019187927246],[2.482426643371582,7.050188064575195,-1.2220624685287476],[2.4378561973571777,7.147011756896973,-0.2046320140361786],[2.111325979232788,7.378675937652588,0.5874464511871338],[2.0000689029693604,7.950705528259277,-0.595825731754303],[2.433171033859253,8.410772323608398,-0.11780398339033127],[2.3782243728637695,8.468542098999023,0.8444681167602539],[1.7954462766647339,8.557205200195312,1.4776698350906372],[1.4570999145507812,8.434192657470703,2.354029417037964],[0.6472896337509155,8.188458442687988,2.262178659439087],[0.03250832483172417,8.606050491333008,2.7742013931274414],[-0.5522511005401611,8.395946502685547,2.2194204330444336],[-1.3662058115005493,8.420257568359375,2.483651638031006],[-1.6792088747024536,8.14314079284668,1.7082802057266235],[-2.398545265197754,8.598665237426758,1.3732343912124634],[-2.1700544357299805,8.378811836242676,0.6083604097366333],[-2.7995409965515137,8.41428279876709,0.048580266535282135],[-2.2902672290802,8.131312370300293,-0.6181186437606812],[-2.3525266647338867,8.576088905334473,-1.4130479097366333],[-1.5762749910354614,8.348807334899902,-1.5725878477096558],[-1.3876714706420898,8.445342063903809,-2.385857343673706],[-0.5753331184387207,8.110359191894531,-2.2966995239257812],[0.09910079091787338,8.555087089538574,-2.7245309352874756],[0.6731991767883301,8.324952125549316,-2.182478189468384],[1.538444995880127,8.444877624511719,-2.0594849586486816],[2.0671756267547607,8.403566360473633,-1.3286898136138916],[2.308972120285034,8.530608177185059,-0.32195761799812317],[1.9874509572982788,8.731541633605957,0.48073580861091614],[1.4845373630523682,9.239972114562988,-0.5232804417610168],[1.5956019163131714,9.46645736694336,0.4588147699832916],[1.1252596378326416,9.281672477722168,1.5181740522384644],[0.1641647070646286,9.482527732849121,1.8520549535751343],[-0.8873077034950256,9.279670715332031,1.7083450555801392],[-1.5887608528137207,9.510747909545898,1.0100245475769043],[-1.881980061531067,9.27827262878418,0.026639945805072784],[-1.6104130744934082,9.496158599853516,-0.9254987835884094],[-0.8134258985519409,9.313295364379883,-1.6667664051055908],[0.23613330721855164,9.458020210266113,-1.7871543169021606],[1.2693504095077515,9.409228324890137,-1.1538678407669067],[1.481530785560608,9.512413024902344,0.0672573670744896],[0.764883816242218,10.232793807983398,-0.29311254620552063],[0.6111859679222107,10.236456871032715,0.8565598726272583],[-0.5166442394256592,10.255976676940918,0.9539554715156555],[-1.0370893478393555,10.259760856628418,-0.041828472167253494],[-0.3055453896522522,10.252521514892578,-0.9250103235244751],[0.7152873873710632,10.279850006103516,-0.3652026951313019],[0.02252264693379402,10.896331787109375,0.06014366075396538]]";
                    // const SHAPE_DONE: &str = "[[0.31830987334251404,0.0,0.0],[1.2370730638504028,0.0,1.1507141590118408],[-0.05424351990222931,0.12463110685348511,1.4680960178375244],[-1.0995595455169678,0.1976633220911026,0.4841819107532501],[-0.827969491481781,0.2586750090122223,-0.9541818499565125],[0.5783724784851074,0.34764325618743896,-1.4283552169799805],[1.7515697479248047,0.4924214482307434,-0.4655873477458954],[2.388977289199829,0.6199695467948914,0.8777152299880981],[1.8412466049194336,0.8141656517982483,2.1230552196502686],[0.5313458442687988,0.9049551486968994,2.5818612575531006],[-0.777789831161499,0.9755407571792603,2.4068286418914795],[-1.8285654783248901,1.0029226541519165,1.4971696138381958],[-2.315776824951172,1.022011399269104,0.2717059552669525],[-2.095008611679077,1.0412347316741943,-1.1003527641296387],[-1.2777618169784546,1.0671888589859009,-2.126178026199341],[0.0355851985514164,1.1186192035675049,-2.5940473079681396],[1.3449687957763672,1.2077690362930298,-2.3659873008728027],[2.423621416091919,1.3509947061538696,-1.4531338214874268],[2.837824821472168,1.5429099798202515,-0.14910170435905457],[2.851050853729248,1.9719066619873047,1.2690303325653076],[1.9860206842422485,2.259855031967163,2.459500551223755],[0.5543471574783325,2.374823570251465,2.954221725463867],[-0.9508198499679565,2.4494898319244385,2.674992799758911],[-2.130542755126953,2.477400541305542,1.6803902387619019],[-2.681373357772827,2.4920852184295654,0.2390804886817932],[-2.4447922706604004,2.5046815872192383,-1.2858549356460571],[-1.4681349992752075,2.5274009704589844,-2.4745986461639404],[-0.011359307914972305,2.579657793045044,-2.983670949935913],[1.502658486366272,2.6762125492095947,-2.660979747772217],[2.6556246280670166,2.8251760005950928,-1.6097482442855835],[3.1579580307006836,3.033729314804077,-0.127439945936203],[3.0081310272216797,3.50546932220459,1.3843592405319214],[2.022430181503296,3.8123855590820312,2.5823280811309814],[0.5082353353500366,3.931138038635254,3.0729925632476807],[-1.059136986732483,3.999481678009033,2.723787784576416],[-2.281843662261963,4.02581787109375,1.6627036333084106],[-2.818337917327881,4.035342693328857,0.1363353133201599],[-2.5435688495635986,4.042794227600098,-1.4592828750610352],[-1.4832459688186646,4.059289932250977,-2.6761202812194824],[0.05554329603910446,4.107248306274414,-3.1795473098754883],[1.6290814876556396,4.198818206787109,-2.769335985183716],[2.7983927726745605,4.340144634246826,-1.611478567123413],[3.263047933578491,4.526867866516113,-0.013161029666662216],[3.2092318534851074,5.070666313171387,1.5990915298461914],[2.135251045227051,5.392273426055908,2.799488067626953],[0.42288514971733093,5.487717628479004,3.3420474529266357],[-1.239152193069458,5.553050994873047,2.8832218647003174],[-2.6252524852752686,5.5594587326049805,1.6833460330963135],[-3.1119484901428223,5.56754732131958,0.00632540974766016],[-2.7864067554473877,5.5507354736328125,-1.8038123846054077],[-1.5659395456314087,5.562995910644531,-3.0645692348480225],[0.17973700165748596,5.5761637687683105,-3.670236349105835],[1.8823000192642212,5.667629241943359,-3.1493542194366455],[3.240326404571533,5.781136512756348,-1.8660669326782227],[3.325857162475586,5.91606330871582,-0.04799496755003929],[1.7860134840011597,5.582504749298096,0.9414116144180298],[0.10973969101905823,5.59566593170166,1.5395134687423706],[-1.2737197875976562,5.632177829742432,0.46249157190322876],[-1.065815806388855,5.659928321838379,-1.2537956237792969],[0.47536084055900574,5.718857288360596,-1.9469321966171265],[1.8402178287506104,5.757357120513916,-1.1402000188827515],[0.35677292943000793,5.616857528686523,-0.2029305100440979],[4.406712532043457,5.875106334686279,1.3630932569503784],[4.354880332946777,5.646071910858154,2.758237838745117],[3.405707597732544,5.803863048553467,3.886274814605713],[2.3747973442077637,5.848057270050049,4.47691011428833],[0.9596548080444336,5.914571762084961,4.91473388671875],[-0.20622923970222473,5.92582893371582,4.909548759460449],[-1.6011813879013062,5.991061210632324,4.5219855308532715],[-2.5940475463867188,5.983084201812744,3.9199788570404053],[-3.7160134315490723,5.974364280700684,2.9421496391296387],[-4.312816619873047,5.949824333190918,1.9234418869018555],[-4.717637538909912,5.969302654266357,0.5229994654655457],[-4.701935291290283,5.943878650665283,-0.6509056091308594],[-4.4322123527526855,5.913585186004639,-2.1184885501861572],[-3.8447442054748535,5.885415554046631,-3.152465581893921],[-2.8267159461975098,5.90233039855957,-4.200700759887695],[-1.779473066329956,5.885887622833252,-4.777148723602295],[-0.3556135594844818,5.86602258682251,-5.258218288421631],[0.8734869956970215,5.8707966804504395,-5.231568813323975],[2.282447338104248,5.9490532875061035,-4.797717094421387],[3.326240301132202,6.007956027984619,-4.072466850280762],[4.425095081329346,6.110310077667236,-2.982102394104004],[4.876824378967285,6.247719764709473,-1.7124576568603516],[4.850254535675049,6.547441482543945,-0.17685452103614807],[4.596287250518799,6.614005088806152,0.9080398082733154],[4.966605186462402,7.233996868133545,1.5249296426773071],[4.5857343673706055,7.1332244873046875,2.6896743774414062],[3.720560312271118,7.254899501800537,3.6627466678619385],[2.6518828868865967,7.315258979797363,4.376964569091797],[1.3900786638259888,7.353131294250488,4.812971591949463],[0.09167474508285522,7.394371509552002,4.914917945861816],[-1.222624659538269,7.434772491455078,4.656736373901367],[-2.4031879901885986,7.456742763519287,4.077608108520508],[-3.4616355895996094,7.426546573638916,3.2365570068359375],[-4.215569496154785,7.429751873016357,2.153611421585083],[-4.687801361083984,7.423868656158447,0.8905648589134216],[-4.779873371124268,7.423853397369385,-0.42615413665771484],[-4.601378917694092,7.368070602416992,-1.7666305303573608],[-4.035166263580322,7.365202903747559,-2.958143711090088],[-3.1729652881622314,7.352221488952637,-3.996039390563965],[-2.066405773162842,7.360715389251709,-4.723149299621582],[-0.8051857948303223,7.308665752410889,-5.231575012207031],[0.5266871452331543,7.339345932006836,-5.300482749938965],[1.861006498336792,7.3833818435668945,-5.011128902435303],[3.034817695617676,7.463374614715576,-4.34440279006958],[4.067063808441162,7.516398906707764,-3.423534870147705],[4.68707799911499,7.658304214477539,-2.2228217124938965],[4.8613810539245605,7.876749515533447,-0.8891631364822388],[4.75483512878418,8.017306327819824,0.3866981565952301],[4.524994373321533,8.596784591674805,1.404019832611084],[4.177597522735596,8.477561950683594,2.201880931854248],[3.2945709228515625,8.55907154083252,3.0765953063964844],[2.5378758907318115,8.594346046447754,3.6250929832458496],[1.333307147026062,8.634824752807617,4.0536088943481445],[0.3611309826374054,8.660287857055664,4.178673267364502],[-0.8940442800521851,8.709346771240234,3.9721786975860596],[-1.813599705696106,8.72137451171875,3.564746856689453],[-2.853663444519043,8.710662841796875,2.8081016540527344],[-3.460538148880005,8.705236434936523,2.0006911754608154],[-3.9612159729003906,8.713000297546387,0.8271974325180054],[-4.061318874359131,8.704172134399414,-0.175095334649086],[-3.9652318954467773,8.669843673706055,-1.455802321434021],[-3.565117597579956,8.65544605255127,-2.375913381576538],[-2.808321714401245,8.658095359802246,-3.3996849060058594],[-1.987288475036621,8.651498794555664,-3.9774575233459473],[-0.8421168327331543,8.626288414001465,-4.554574012756348],[0.17211394011974335,8.635296821594238,-4.6480937004089355],[1.4311139583587646,8.693777084350586,-4.465749263763428],[2.3511452674865723,8.743557929992676,-3.9935555458068848],[3.3765366077423096,8.812823295593262,-3.2217037677764893],[3.921025037765503,8.922697067260742,-2.3003711700439453],[4.067803382873535,9.114948272705078,-1.0271105766296387],[3.9294116497039795,9.217751502990723,0.15561915934085846],[3.2134110927581787,9.28063678741455,1.3565586805343628],[2.1799070835113525,9.289419174194336,2.321105480194092],[0.6903526782989502,9.32392406463623,2.8489809036254883],[-0.8240306377410889,9.361369132995605,2.5984444618225098],[-2.1316447257995605,9.363473892211914,1.6410696506500244],[-2.7376153469085693,9.363951683044434,0.20928384363651276],[-2.6080682277679443,9.340729713439941,-1.4041727781295776],[-1.685996413230896,9.340899467468262,-2.646064043045044],[-0.2480684071779251,9.342347145080566,-3.366528272628784],[1.277744174003601,9.414711952209473,-3.14687180519104],[2.554823398590088,9.527653694152832,-2.181206703186035],[2.730499505996704,9.68076229095459,-0.5991960763931274],[1.719118356704712,9.72574520111084,0.7695055603981018],[0.16706985235214233,9.766704559326172,1.3531570434570312],[-1.1691503524780273,9.790177345275879,0.4086960256099701],[-1.0727527141571045,9.791757583618164,-1.1916779279708862],[0.2765865921974182,9.83859920501709,-1.9448635578155518],[1.4997940063476562,10.057331085205078,-1.2588261365890503],[0.2977505326271057,9.947281837463379,-0.264589786529541]]";
                    // let primary = Pointcloud::from_points_str(SHAPE_INIT);
                    let primary = examples::ergogrzib();
                    let secondary = Pointcloud::from_points_file("models/grzib10cloud1000.json");
                    let sim = PlushieSimulation::with_secondary(primary, secondary);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                6 => {
                    let plushie =
                        Pointcloud::from_points_file("model_preprocessing/pointcloud.json");
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                7 => {
                    let mut plushie = examples::ergogrzib();
                    plushie.params = Params::handpicked_for_grzib();
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                8 => rocket_server::main(),
                9 => {
                    fn genome_to_actions(genome: &Vec<u8>) -> Vec<Action> {
                        use Action::*;
                        let mut result = vec![MR(6)];

                        for gene in genome {
                            result.push(match *gene {
                                0 => Sc,
                                1 => Inc,
                                2 => Dec,
                                3 => FLO,
                                4 => BLO,
                                _ => panic!("Unrecognized gene: {}", gene),
                            });
                        }

                        result.push(FO);
                        result
                    }
                    let genome: Vec<u8> = serde_json::from_str("[3, 1, 1, 3, 4, 1, 0, 4, 3, 2, 1, 4, 0, 0, 4, 3, 0, 2, 4, 2, 1, 0, 0, 0, 3, 0, 0, 2, 0, 2, 4, 3, 3, 3, 0, 1, 1, 1, 4, 2, 4, 0, 1, 3, 2, 4, 4, 3, 2, 3, 4, 3, 4, 1, 4, 3, 4, 4, 3, 1, 2, 4, 4, 4, 1, 3, 0, 4, 4, 3, 2, 0, 3, 1, 3, 4, 0, 0, 4, 1, 0, 0, 3, 1, 3, 3, 2, 2, 1, 3, 2, 0, 2, 0, 1, 0, 1, 0, 3, 4, 3, 0, 1, 1, 0, 1, 4, 4, 2, 2, 3, 1, 1, 0, 3, 4, 1, 1, 4, 0, 0, 3, 0, 3, 4, 0, 3, 1, 2, 1, 0, 1, 2]").unwrap();
                    let actions = genome_to_actions(&genome);
                    let plushie =
                        Plushie::from_flow(ErgoFlow::from(actions), Params::handpicked_for_grzib())
                            .unwrap();
                    let sim = PlushieSimulation::from(plushie);
                    serve_websocket(sim, "127.0.0.1:8080");
                }
                _ => {}
            }
            println!(":)");
            println!(":)");
        }
        Genetic(genetic) => {
            let suite = &genetic.suite;
            println!("Selected suite: {suite}");
            unimplemented!();
            // run_benchmark(&suite, &genetic);
        }
        FromPattern { pattern, stl, ws } => {
            let pattern = {
                let content = fs::read_to_string(&pattern).unwrap();
                match Pattern::parse(&content) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("{e}");
                        return;
                    }
                }
            };
            let mut params: Params = Default::default();
            params.update(&pattern.meta);
            let plushie = Plushie::from_flow(pattern, params).unwrap();

            if stl.is_some() && ws || stl.is_none() && !ws {
                println!("use either --stl or --ws");
                return;
            }

            if let Some(_stl_path) = stl {
                unimplemented!()
                // plushie.animate();
                // save_mesh(stl_path.to_str().unwrap(), plushie.to_mesh());
            } else if ws {
                let sim = PlushieSimulation::from(plushie);
                serve_websocket(sim, "127.0.0.1:8080");
            }
        }
    }
}
