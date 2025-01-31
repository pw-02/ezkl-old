import json
import torch 
from torch import nn

class Circuit(nn.Module):
    def __init__(self):
        super(Circuit, self).__init__()

    def forward(self, x):
        return torch.special.erf(x)

def main():
    torch_model = Circuit()
    # Input to the model
    shape = [3]
    x = torch.rand(1,*shape, requires_grad=True)
    torch_out = torch_model(x)
    # Export the model
    torch.onnx.export(torch_model,               # model being run
                      x,                   # model input (or a tuple for multiple inputs)
                      "network.onnx",            # where to save the model (can be a file or file-like object)
                      export_params=True,        # store the trained parameter weights inside the model file
                      opset_version=10,          # the ONNX version to export the model to
                      do_constant_folding=True,  # whether to execute constant folding for optimization
                      input_names = ['input'],   # the model's input names
                      output_names = ['output'], # the model's output names
                      dynamic_axes={'input' : {0 : 'batch_size'},    # variable length axes
                                    'output' : {0 : 'batch_size'}})

    d = ((x).detach().numpy()).reshape([-1]).tolist()

    data = dict(input_shapes = [shape],
                input_data = [d],
                output_data = [((o).detach().numpy()).reshape([-1]).tolist() for o in torch_out])

    # Serialize data into file:
    json.dump( data, open( "input.json", 'w' ) )

if __name__ == "__main__":
    main()
