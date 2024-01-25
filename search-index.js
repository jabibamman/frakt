var searchIndex = JSON.parse('{\
"cli":{"doc":"","t":"AAEDDNIYNLLLLLLLLLLLLLLLLLLLLMMLLLLLLLLLLLMMLLLMLLMMMLLLLLLLLLLLLLLLLLLLLMMM","n":["operation","parser","CliArgs","CliClientArgs","CliServerArgs","Client","Parser","Parser","Server","augment_args","augment_args","augment_args_for_update","augment_args_for_update","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","command","command","command_for_update","command_for_update","debug","debug","fmt","fmt","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches_mut","from_arg_matches_mut","group_id","group_id","hostname","hostname","into","into","into","open","parse","parse_from","port","port","save","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","try_parse","try_parse_from","try_update_from","type_id","type_id","type_id","update_from","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches_mut","update_from_arg_matches_mut","verbose","verbose","worker_name"],"q":[[0,"cli"],[2,"cli::parser"],[76,"clap_builder::builder::command"],[77,"core::fmt"],[78,"core::fmt"],[79,"clap_builder"],[80,"core::result"],[81,"clap_builder::util::id"],[82,"core::option"],[83,"core::iter::traits::collect"],[84,"clap_builder::error"],[85,"core::any"]],"d":["","","An enumeration representing the possible types of command …","Represents command line arguments for a client in a CLI …","Represents command line arguments for a server in a CLI …","","Parse command-line arguments into <code>Self</code>.","Generates the <code>Parser</code> implementation.","","","","","","","","","","","","","","","","","","","","","","Optional: Add a flag to enable/disable debug mode. …","Optional: Add a flag to enable/disable debug mode. …","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Optional: The hostname of the client. Default: “localhost…","The hostname of the server. Default: “localhost”","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Optional: Add a flag to enable/disable opening the browser.","Parse from <code>std::env::args_os()</code>, exit on error.","Parse from iterator, exit on error.","Optional: The port number to connect on. Default: 8787","The port number the server listens on. Default: 8787","Optional: Add a flag to save the image to a file. Default: …","","","","","","","","","","Parse from <code>std::env::args_os()</code>, return Err on error.","Parse from iterator, return Err on error.","Update from iterator, return Err on error.","","","","Update from iterator, exit on error.","","","","","Optional: Add a flag to enable/disable logging. Default: …","Optional: Add a flag to enable/disable logging. Default: …","Optional: The name of the worker. Default: “worker”"],"i":[0,0,0,0,0,4,0,0,4,2,3,2,3,2,3,4,2,3,4,2,3,4,2,3,4,2,3,2,3,2,3,2,3,2,3,4,2,3,2,3,2,3,2,3,2,3,4,2,16,16,2,3,2,2,3,4,2,3,4,2,3,4,16,16,16,2,3,4,16,2,3,2,3,2,3,2],"f":[0,0,0,0,0,0,0,0,0,[1,1],[1,1],[1,1],[1,1],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[2,2],[3,3],[4,4],[[-1,-2],5,[],[]],[[-1,-2],5,[],[]],[[-1,-2],5,[],[]],[[],1],[[],1],[[],1],[[],1],0,0,[[2,6],7],[[3,6],7],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[8,[[10,[2,9]]]],[8,[[10,[3,9]]]],[8,[[10,[2,9]]]],[8,[[10,[3,9]]]],[[],[[12,[11]]]],[[],[[12,[11]]]],0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,[[],-1,[]],[-1,-2,13,[]],0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[[],[[10,[-1,14]]],[]],[-1,[[10,[-2,14]]],13,[]],[[-1,-2],[[10,[5,14]]],[],13],[-1,15,[]],[-1,15,[]],[-1,15,[]],[[-1,-2],5,[],13],[[2,8],[[10,[5,9]]]],[[3,8],[[10,[5,9]]]],[[2,8],[[10,[5,9]]]],[[3,8],[[10,[5,9]]]],0,0,0],"c":[],"p":[[3,"Command",76],[3,"CliClientArgs",2],[3,"CliServerArgs",2],[4,"CliArgs",2],[15,"tuple"],[3,"Formatter",77],[6,"Result",77],[3,"ArgMatches",78],[6,"Error",79],[4,"Result",80],[3,"Id",81],[4,"Option",82],[8,"IntoIterator",83],[3,"Error",84],[3,"TypeId",85],[8,"Parser",2]],"b":[]},\
"client":{"doc":"","t":"AAFAFFFFFFFFFFF","n":["fractal_generation","image","main","networking","color","generate_fractal_set","hsl_to_rgb","open_image","connect_to_server","convert_to_pixel_data","process_fragment_task","receive_fragment_task","save_fractal_image","send_fragment_result","send_request"],"q":[[0,"client"],[4,"client::fractal_generation"],[7,"client::image"],[8,"client::networking"],[15,"shared::types::error"],[16,"core::result"],[17,"shared::types::pixel_intensity"],[18,"shared::types::messages"],[19,"image::color"],[20,"alloc::vec"],[21,"image::buffer_"],[22,"shared::types::color"],[23,"shared::types::color"],[24,"cli::parser"],[25,"std::net::tcp"],[26,"shared::types::pixel_data"],[27,"core::option"],[28,"std::io::error"]],"d":["","","","","Generates a color based on the provided pixel intensity.","Generates an image of a Fractal Type based on the provided …","Convert a color from HSL to RGB","Opens an image file with the default image viewer of the …","Connect to a server at the specified address.","Convert a <code>Vec&lt;u8&gt;</code> to a <code>PixelData</code> struct.","Process a <code>FragmentTask</code> by generating a fractal image and …","Receive a <code>FragmentTask</code> from the server.","Save a fractal image to the filesystem.","Send a <code>FragmentResult</code> to the server after generating a …","Send a request to the server."],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[0,0,[[],[[3,[1,2]]]],0,[4,[[6,[5]]]],[7,[[3,[[1,[[10,[[8,[5]],[9,[5]]]],[9,[5]],[9,[4]]]],2]]]],[11,12],[13,[[3,[1,14]]]],[15,[[3,[16,2]]]],[[[9,[5]],7],17],[[7,[9,[5]],15],[[3,[16,2]]]],[16,[[3,[[18,[[1,[7,[9,[5]]]]]],2]]]],[[[10,[[8,[5]],[9,[5]]]],13],[[3,[1,2]]]],[[7,15,17,[9,[5]]],[[3,[16,2]]]],[[16,13],[[19,[1]]]]],"c":[],"p":[[15,"tuple"],[4,"FractalError",15],[4,"Result",16],[3,"PixelIntensity",17],[15,"u8"],[15,"array"],[3,"FragmentTask",18],[3,"Rgb",19],[3,"Vec",20],[3,"ImageBuffer",21],[3,"HSL",22],[3,"RGB",22],[15,"str"],[3,"Error",23],[3,"CliClientArgs",24],[3,"TcpStream",25],[3,"PixelData",26],[4,"Option",27],[6,"Result",23]],"b":[]},\
"complex":{"doc":"","t":"AAAAAAIKKKKKKKKKKKKIKIKKKKIKKKIKIK","n":["complex_operations","fractal_operations","iterated_sinz_impl","julia_descriptor_impl","mandelbrot_descriptor_impl","newtonraphsonz_descriptor_impl","ComplexOperations","abs","add","arg","div","exp","magnitude_squared","mul","new","norm","sin","square","sub","FractalOperations","compute_pixel_intensity","IteratedSinZOperations","c","max_iteration","new","η","JuliaOperations","c","divergence_threshold_square","new","MandelbrotOperations","new","NewtonRaphsonOperations","new"],"q":[[0,"complex"],[6,"complex::complex_operations"],[19,"complex::fractal_operations"],[21,"complex::iterated_sinz_impl"],[26,"complex::julia_descriptor_impl"],[30,"complex::mandelbrot_descriptor_impl"],[32,"complex::newtonraphsonz_descriptor_impl"],[34,"shared::types::complex"],[35,"shared::types::pixel_intensity"]],"d":["","","","","","","Provides a set of operations for complex number arithmetic.","Returns the absolute value of the complex number.","Adds two complex numbers and returns the result.","Returns the argument of the complex number in radians.","Divides two complex numbers and returns the result.","Returns the exponential of the complex number.","Returns the squared magnitude of the complex number.","Multiplies two complex numbers and returns the result.","Constructs a new complex number.","Returns the Euclidean norm (magnitude) of the complex …","Returns the sine of the complex number.","Squares the complex number and returns the result.","Subtracts another complex number from this one and returns …","","Computes the pixel intensity of a complex point.","Provides operations specific to the Iterated Sin(z) …","Returns a reference to the complex number <code>c</code> used in the …","Fixed to 50","Constructs a new <code>IteratedSinZDescriptor</code> with the specified …","Returns the divergence threshold <code>η</code> used in the …","Provides operations specific to the Julia fractal.","Returns a reference to the complex number <code>c</code> used in the …","Returns the square of the divergence threshold.","Constructs a new <code>JuliaDescriptor</code> with the specified …","Trait définissant les opérations de fractale de …","Crée une nouvelle instance du type de fractale de …","","Crée une nouvelle instance du type de fractale."],"i":[0,0,0,0,0,0,0,5,5,5,5,5,5,5,5,5,5,5,5,0,6,0,7,7,7,7,0,8,8,8,0,9,0,10],"f":[0,0,0,0,0,0,0,[-1,1,[]],[[-1,-1],-1,[]],[-1,1,[]],[[-1,-1],-1,[]],[-1,-1,[]],[-1,1,[]],[[-1,-1],-1,[]],[[1,1],-1,[]],[-1,1,[]],[-1,-1,[]],[-1,-1,[]],[[-1,-1],-1,[]],0,[[-1,2,3],4,[]],0,[-1,2,[]],[-1,3,[]],[2,-1,[]],[-1,1,[]],0,[-1,2,[]],[-1,1,[]],[[2,1],-1,[]],0,[[],-1,[]],0,[[],-1,[]]],"c":[],"p":[[15,"f64"],[3,"Complex",34],[15,"u16"],[3,"PixelIntensity",35],[8,"ComplexOperations",6],[8,"FractalOperations",19],[8,"IteratedSinZOperations",21],[8,"JuliaOperations",26],[8,"MandelbrotOperations",30],[8,"NewtonRaphsonOperations",32]],"b":[]},\
"server":{"doc":"","t":"IYIYKAAKAAAAFFFFAAAFFFFF","n":["Deserialize","Deserialize","Serialize","Serialize","deserialize","image","messages","serialize","services","fragment_maker","handler","serialization","create_task_for_request","process_result","handle_client","deserialize_message","reader","server_runner","write","get_response","read_message","run_server","write","write_img"],"q":[[0,"server"],[9,"server::messages"],[12,"server::messages::fragment_maker"],[14,"server::messages::handler"],[15,"server::messages::serialization"],[16,"server::services"],[19,"server::services::reader"],[21,"server::services::server_runner"],[22,"server::services::write"],[24,"core::result"],[25,"serde::de"],[26,"serde::ser"],[27,"shared::types::messages"],[28,"shared::types::messages"],[29,"std::io::error"],[30,"shared::types::messages"],[31,"alloc::string"],[32,"alloc::vec"]],"d":["A <strong>data structure</strong> that can be deserialized from any data …","","A <strong>data structure</strong> that can be serialized into any data …","","Deserialize this value from the given Serde deserializer.","","","Serialize this value into the given Serde serializer.","","","","","","","Handles a client TCP stream.","Deserializes a JSON string into a <code>Message</code> enum variant.","","","","","Reads a message from a TCP stream, parsing and returning …","Starts a TCP server on the specified address.","Write a string to a stream","Prepare a message for sending."],"i":[0,0,0,0,16,0,0,17,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[0,0,0,0,[-1,[[1,[-2]]],2,[]],0,0,[[-1,-2],1,[],3],0,0,0,0,[4,5],[6,7],[8,[[9,[7]]]],[10,[[12,[11]]]],0,0,0,[8,[[9,[[7,[13,[15,[14]]]]]]]],[8,[[9,[[7,[13,[15,[14]]]]]]]],[10,[[9,[7]]]],[[8,10],[[9,[7]]]],[[8,10,[15,[14]]],[[9,[7]]]]],"c":[],"p":[[4,"Result",24],[8,"Deserializer",25],[8,"Serializer",26],[3,"FragmentRequest",27],[3,"FragmentTask",27],[3,"FragmentResult",27],[15,"tuple"],[3,"TcpStream",28],[6,"Result",29],[15,"str"],[4,"Message",27],[6,"Result",30],[3,"String",31],[15,"u8"],[3,"Vec",32],[8,"Deserialize",0],[8,"Serialize",0]],"b":[]},\
"shared":{"doc":"","t":"AAAFAAAAAAAAAAAADDMLLLLLLLLLLLLLLLLMMLLLLMMMLLLLLLLLLDLLLLLLLLLLLMLLMLLLLLLNENNNNNNNNLLLLLLLLLLLLLLLLLLNENNEENNNNLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLDENDNDNDNDNDLLLLLLLLLLLLLLMMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLDNDNDNELLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLMMLLLLLLLLMMLLLMMMMMLLLLLLLLLLLLLLLLLLLLLLLLLLMDLLLLMLLLLLLLLLLMLLLLLLDLLLLMLLLLLLLLLLLLLMDLLLLLLLLLLLLLLLLLLLMMDLLLLLLLLLLLLLMMLLLLLLDLLLLLLLLLLLLLMMLLLLLLDLLLLMLLLLLLLLLMLLLLLLAAAAAFFFFFIKKKIKKKIKKKF","n":["logger","types","utils","init_logger","color","complex","error","filesystem","fractal_descriptor","messages","pixel_data","pixel_intensity","point","range","resolution","u8data","HSL","RGB","b","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","deref","deref","deref_mut","deref_mut","drop","drop","eq","fmt","from","from","g","h","init","init","into","into","l","r","s","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","Complex","borrow","borrow_mut","clone","clone_into","deref","deref_mut","deserialize","drop","eq","fmt","from","im","init","into","re","serialize","to_owned","try_from","try_into","type_id","vzip","ConnectionError","FractalError","Image","Io","NotFound","Other","PathConversion","SerializationError","TaskNotSet","UnsupportedOperation","borrow","borrow_mut","deref","deref_mut","drop","fmt","fmt","from","from","from","from","init","into","to_string","try_from","try_into","type_id","vzip","Current","DirType","Directory","File","FileExtension","FileType","JPEG","JPG","PNG","Workspace","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","deref","deref","deref","deref_mut","deref_mut","deref_mut","drop","drop","drop","from","from","from","init","init","init","into","into","into","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","FractalDescriptor","FractalType","IteratedSinZ","IteratedSinZDescriptor","Julia","JuliaDescriptor","Mandelbrot","MandelbrotDescriptor","NewtonRaphsonZ3","NewtonRaphsonZ3Descriptor","NewtonRaphsonZ4","NewtonRaphsonZ4Descriptor","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","c","c","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","deref","deref","deref","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","divergence_threshold_square","drop","drop","drop","drop","drop","drop","drop","eq","eq","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fractal_type","from","from","from","from","from","from","from","init","init","init","init","init","init","init","into","into","into","into","into","into","into","serialize","serialize","serialize","serialize","serialize","serialize","serialize","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","vzip","vzip","vzip","FragmentRequest","FragmentRequest","FragmentResult","FragmentResult","FragmentTask","FragmentTask","Message","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","drop","drop","drop","drop","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fractal","from","from","from","from","id","id","init","init","init","init","into","into","into","into","max_iteration","maximal_work_load","new","new","new","pixels","range","range","resolution","resolution","serialize","serialize","serialize","serialize","serialize","serialize","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","worker_name","PixelData","borrow","borrow_mut","clone","clone_into","count","deref","deref_mut","deserialize","drop","eq","fmt","from","init","into","new","offset","serialize","to_owned","try_from","try_into","type_id","vzip","PixelIntensity","borrow","borrow_mut","clone","clone_into","count","deref","deref_mut","drop","eq","fmt","from","init","into","to_owned","try_from","try_into","type_id","vzip","zn","Point","borrow","borrow_mut","clone","clone_into","deref","deref_mut","deserialize","drop","eq","fmt","from","init","into","serialize","to_owned","try_from","try_into","type_id","vzip","x","y","Range","borrow","borrow_mut","clone","clone_into","deref","deref_mut","deserialize","drop","eq","fmt","from","init","into","max","min","serialize","to_owned","try_from","try_into","type_id","vzip","Resolution","borrow","borrow_mut","clone","clone_into","deref","deref_mut","deserialize","drop","eq","fmt","from","init","into","nx","ny","serialize","to_owned","try_from","try_into","type_id","vzip","U8Data","borrow","borrow_mut","clone","clone_into","count","deref","deref_mut","deserialize","drop","eq","fmt","from","init","into","offset","serialize","to_owned","try_from","try_into","type_id","vzip","filesystem","fragment_request_impl","fragment_result_impl","fragment_task_impl","type_of","dir_exists","get_dir_path_buf","get_extension_str","get_file_path","get_workspace_dir","FragmentRequestOperation","deserialize","new","serialize","FragmentResultOperation","deserialize","new","serialize","FragmentTaskOperation","deserialize","new","serialize","type_of"],"q":[[0,"shared"],[3,"shared::logger"],[4,"shared::types"],[16,"shared::types::color"],[53,"shared::types::complex"],[75,"shared::types::error"],[103,"shared::types::filesystem"],[149,"shared::types::fractal_descriptor"],[298,"shared::types::messages"],[399,"shared::types::pixel_data"],[422,"shared::types::pixel_intensity"],[442,"shared::types::point"],[464,"shared::types::range"],[486,"shared::types::resolution"],[508,"shared::types::u8data"],[530,"shared::utils"],[535,"shared::utils::filesystem"],[540,"shared::utils::fragment_request_impl"],[544,"shared::utils::fragment_result_impl"],[548,"shared::utils::fragment_task_impl"],[552,"shared::utils::type_of"],[553,"core::result"],[554,"core::fmt"],[555,"core::fmt"],[556,"serde::de"],[557,"serde::ser"],[558,"serde_json::error"],[559,"alloc::string"],[560,"std::io::error"],[561,"std::path"],[562,"std::io::error"]],"d":["","","","","","","","","","","","","","","","","HSL : Hue, Saturation, Lightness","RGB : Red, Green, Blue","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","A <code>Complex</code> number with real (<code>re</code>) and imaginary (<code>im</code>) parts.","","","","","","","","","","","Returns the argument unchanged.","","","Calls <code>U::from(self)</code>.","","","","","","","","Error related to network connection.","Enum representing various types of errors that can occur …","Error related to image processing.","Error related to Input/Output operations.","Error indicating that a required item was not found.","Generic error for other cases.","Error occurring when a path cannot be converted to a …","Error related to data serialization.","Error indicating that a required task was not set.","Error representing an unsupported operation.","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","Represents the types of directories used in the …","","","Enumerates the supported image file extensions.","Specifies the types of files or entities in a file system.","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","General descriptor for a fractal, encompassing different …","Represents the type of fractal to be generated.","","Describes parameters specific to a Mandelbrot fractal.","","Describes parameters specific to a Julia fractal.","","Describes parameters specific to a Mandelbrot fractal.","","Describes parameters specific to a Newton-Raphson z3 …","","Describes parameters specific to a Newton-Raphson z3 …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Represents a request for a fragment of work from a worker.","","Represents the result of a fragment computation by a …","","Describes a task assigned to a worker for fractal …","","An enumeration representing different types of messages in …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Represents data associated with a set of pixels in an …","","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","Represents the intensity of a pixel in fractal rendering, …","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","Represents a point in a two-dimensional space.","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","Defines a rectangular range in a two-dimensional space, …","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","Represents the resolution of an image or a grid, defined …","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","Represents a segment of data, typically used for handling …","","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","Checks if a given path string represents an existing …","Returns a <code>PathBuf</code> representing a directory path.","Maps a <code>FileExtension</code> enum variant to its corresponding …","Generates a file path string with a random component in …","Obtains the workspace directory. Typically the current …","","","","Serializes a <code>FragmentRequest</code> into a JSON string.","","","","","","Deserializes a JSON string into a <code>FragmentTask</code>.","","Serializes a <code>FragmentTask</code> into a JSON string.","Returns the name of a type."],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,40,5,40,5,5,5,40,5,40,5,40,5,5,5,40,5,5,40,40,5,40,5,40,5,40,5,40,5,40,5,40,5,40,5,0,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,3,0,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,41,0,42,42,0,0,37,37,37,41,41,42,37,41,42,37,41,42,37,41,42,37,41,42,37,41,42,37,41,42,37,41,42,37,41,42,37,41,42,37,41,42,37,41,42,37,0,0,16,0,16,0,16,0,16,0,16,0,16,17,18,19,20,21,22,16,17,18,19,20,21,22,17,18,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,17,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,20,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,16,17,18,19,20,21,22,0,26,0,26,0,26,0,23,24,25,26,23,24,25,26,23,24,25,26,23,24,25,26,23,24,25,26,23,24,25,26,23,23,24,24,25,25,23,24,25,26,23,24,25,26,23,24,25,26,24,23,24,25,26,24,25,23,24,25,26,23,24,25,26,24,23,23,24,25,25,24,25,24,25,23,23,24,24,25,25,23,24,25,26,23,24,25,26,23,24,25,26,23,24,25,26,23,24,25,26,23,0,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,33,0,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,0,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,0,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,32,0,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,31,0,29,29,29,29,29,29,29,29,29,29,29,29,29,29,29,29,29,29,29,29,29,0,0,0,0,0,0,0,0,0,0,0,43,43,43,0,44,44,44,0,45,45,45,0],"f":[0,0,0,[[1,1],[[4,[2,3]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[5,5],[[-1,-2],2,[],[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,2],[6,2],[[5,5],1],[[5,7],8],[-1,-1,[]],[-1,-1,[]],0,0,[[],6],[[],6],[-1,-2,[],[]],[-1,-2,[],[]],0,0,0,[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],[-1,-2,[],[]],[-1,-2,[],[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[10,10],[[-1,-2],2,[],[]],[6,-1,[]],[6,-1,[]],[-1,[[4,[10]]],11],[6,2],[[10,10],1],[[10,7],8],[-1,-1,[]],0,[[],6],[-1,-2,[],[]],0,[[10,-1],4,12],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,-2,[],[]],0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[6,-1,[]],[6,-1,[]],[6,2],[[3,7],8],[[3,7],8],[13,3],[14,3],[15,3],[-1,-1,[]],[[],6],[-1,-2,[],[]],[-1,14,[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,-2,[],[]],0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,2],[6,2],[6,2],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[[],6],[[],6],[[],6],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,[16,16],[17,17],[18,18],[19,19],[20,20],[21,21],[22,22],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[-1,[[4,[16]]],11],[-1,[[4,[17]]],11],[-1,[[4,[18]]],11],[-1,[[4,[19]]],11],[-1,[[4,[20]]],11],[-1,[[4,[21]]],11],[-1,[[4,[22]]],11],0,[6,2],[6,2],[6,2],[6,2],[6,2],[6,2],[6,2],[[16,16],1],[[17,17],1],[[18,18],1],[[19,19],1],[[20,20],1],[[21,21],1],[[22,22],1],[[16,7],8],[[17,7],8],[[18,7],8],[[19,7],8],[[20,7],8],[[21,7],8],[[22,7],8],0,[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[[16,-1],4,12],[[17,-1],4,12],[[18,-1],4,12],[[19,-1],4,12],[[20,-1],4,12],[[21,-1],4,12],[[22,-1],4,12],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[23,23],[24,24],[25,25],[26,26],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[[-1,-2],2,[],[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[6,-1,[]],[27,[[4,[23,13]]]],[-1,[[4,[23]]],11],[27,[[4,[24,13]]]],[-1,[[4,[24]]],11],[-1,[[4,[25]]],11],[27,[[4,[25,13]]]],[6,2],[6,2],[6,2],[6,2],[[23,23],1],[[24,24],1],[[25,25],1],[[26,26],1],[[23,7],8],[[24,7],8],[[25,7],8],[[26,7],8],0,[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],[-1,-1,[]],0,0,[[],6],[[],6],[[],6],[[],6],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,[[14,28],23],[[29,20,30,31,32],24],[[29,31,32,33],25],0,0,0,0,0,[23,[[4,[14,13]]]],[[23,-1],4,12],[24,[[4,[14,13]]]],[[24,-1],4,12],[[25,-1],4,12],[25,[[4,[14,13]]]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,9,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],0,0,[-1,-2,[],[]],[-1,-2,[],[]],[33,33],[[-1,-2],2,[],[]],0,[6,-1,[]],[6,-1,[]],[-1,[[4,[33]]],11],[6,2],[[33,33],1],[[33,7],8],[-1,-1,[]],[[],6],[-1,-2,[],[]],[[28,28],33],0,[[33,-1],4,12],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,-2,[],[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[34,34],[[-1,-2],2,[],[]],0,[6,-1,[]],[6,-1,[]],[6,2],[[34,34],1],[[34,7],8],[-1,-1,[]],[[],6],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,-2,[],[]],0,0,[-1,-2,[],[]],[-1,-2,[],[]],[35,35],[[-1,-2],2,[],[]],[6,-1,[]],[6,-1,[]],[-1,[[4,[35]]],11],[6,2],[[35,35],1],[[35,7],8],[-1,-1,[]],[[],6],[-1,-2,[],[]],[[35,-1],4,12],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,-2,[],[]],0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[32,32],[[-1,-2],2,[],[]],[6,-1,[]],[6,-1,[]],[-1,[[4,[32]]],11],[6,2],[[32,32],1],[[32,7],8],[-1,-1,[]],[[],6],[-1,-2,[],[]],0,0,[[32,-1],4,12],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,-2,[],[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[31,31],[[-1,-2],2,[],[]],[6,-1,[]],[6,-1,[]],[-1,[[4,[31]]],11],[6,2],[[31,31],1],[[31,7],8],[-1,-1,[]],[[],6],[-1,-2,[],[]],0,0,[[31,-1],4,12],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,-2,[],[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[29,29],[[-1,-2],2,[],[]],0,[6,-1,[]],[6,-1,[]],[-1,[[4,[29]]],11],[6,2],[[29,29],1],[[29,7],8],[-1,-1,[]],[[],6],[-1,-2,[],[]],0,[[29,-1],4,12],[-1,-2,[],[]],[-1,[[4,[-2]]],[],[]],[-1,[[4,[-2]]],[],[]],[-1,9,[]],[-1,-2,[],[]],0,0,0,0,0,[27,1],[[],[[4,[36,3]]]],[37,27],[[27,36,27],[[4,[14,3]]]],[[],[[38,[36]]]],0,[27,[[4,[-1,13]]],39],[[14,28],-1,[]],[-1,[[4,[14,13]]],[]],0,[27,[[4,[-1,13]]],39],[[29,31,32,33],-1,[]],[-1,[[4,[14,13]]],[]],0,[27,[[4,[-1,13]]],39],[[29,20,30,31,32],-1,[]],[-1,[[4,[14,13]]],[]],[-1,27,[]]],"c":[],"p":[[15,"bool"],[15,"tuple"],[4,"FractalError",75],[4,"Result",553],[3,"RGB",16],[15,"usize"],[3,"Formatter",554],[6,"Result",554],[3,"TypeId",555],[3,"Complex",53],[8,"Deserializer",556],[8,"Serializer",557],[3,"Error",558],[3,"String",559],[3,"Error",560],[4,"FractalType",149],[3,"JuliaDescriptor",149],[3,"IteratedSinZDescriptor",149],[3,"MandelbrotDescriptor",149],[3,"FractalDescriptor",149],[3,"NewtonRaphsonZ3Descriptor",149],[3,"NewtonRaphsonZ4Descriptor",149],[3,"FragmentRequest",298],[3,"FragmentTask",298],[3,"FragmentResult",298],[4,"Message",298],[15,"str"],[15,"u32"],[3,"U8Data",508],[15,"u16"],[3,"Resolution",486],[3,"Range",464],[3,"PixelData",399],[3,"PixelIntensity",422],[3,"Point",442],[3,"PathBuf",561],[4,"FileExtension",103],[6,"Result",560],[8,"Sized",562],[3,"HSL",16],[4,"DirType",103],[4,"FileType",103],[8,"FragmentRequestOperation",540],[8,"FragmentResultOperation",544],[8,"FragmentTaskOperation",548]],"b":[[90,"impl-Display-for-FractalError"],[91,"impl-Debug-for-FractalError"],[92,"impl-From%3CError%3E-for-FractalError"],[93,"impl-From%3CString%3E-for-FractalError"],[94,"impl-From%3CError%3E-for-FractalError"],[329,"impl-FragmentRequestOperation-for-FragmentRequest"],[330,"impl-Deserialize%3C\'de%3E-for-FragmentRequest"],[331,"impl-FragmentTaskOperation-for-FragmentTask"],[332,"impl-Deserialize%3C\'de%3E-for-FragmentTask"],[333,"impl-Deserialize%3C\'de%3E-for-FragmentResult"],[334,"impl-FragmentResultOperation-for-FragmentResult"],[372,"impl-FragmentRequestOperation-for-FragmentRequest"],[373,"impl-Serialize-for-FragmentRequest"],[374,"impl-FragmentTaskOperation-for-FragmentTask"],[375,"impl-Serialize-for-FragmentTask"],[376,"impl-Serialize-for-FragmentResult"],[377,"impl-FragmentResultOperation-for-FragmentResult"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
