//! Contains Miden assembly of the [Tip5 permutation](https://eprint.iacr.org/2023/107.pdf).
//! The binary runs the permutation once on statically defined input.

use miden_stdlib::StdLibrary;
use miden_vm::prove;
use miden_vm::verify;
use miden_vm::Assembler;
use miden_vm::Kernel;
use miden_vm::MemAdviceProvider;
use miden_vm::ProgramInfo;
use miden_vm::ProofOptions;
use miden_vm::StackInputs;

/// The [Tip5](https://eprint.iacr.org/2023/107.pdf) permutation.
///
/// While this is technically not a Miden library, it should be relatively easy to convert it to
/// one.
pub const TIP5_LIB: &str = "
    proc.tip5_init
        push.0   mem_store.0
        push.7   mem_store.1
        push.26  mem_store.2
        push.63  mem_store.3
        push.124 mem_store.4
        push.215 mem_store.5
        push.85  mem_store.6
        push.254 mem_store.7
        push.214 mem_store.8
        push.228 mem_store.9
        push.45  mem_store.10
        push.185 mem_store.11
        push.140 mem_store.12
        push.173 mem_store.13
        push.33  mem_store.14
        push.240 mem_store.15
        push.29  mem_store.16
        push.177 mem_store.17
        push.176 mem_store.18
        push.32  mem_store.19
        push.8   mem_store.20
        push.110 mem_store.21
        push.87  mem_store.22
        push.202 mem_store.23
        push.204 mem_store.24
        push.99  mem_store.25
        push.150 mem_store.26
        push.106 mem_store.27
        push.230 mem_store.28
        push.14  mem_store.29
        push.235 mem_store.30
        push.128 mem_store.31
        push.213 mem_store.32
        push.239 mem_store.33
        push.212 mem_store.34
        push.138 mem_store.35
        push.23  mem_store.36
        push.130 mem_store.37
        push.208 mem_store.38
        push.6   mem_store.39
        push.44  mem_store.40
        push.71  mem_store.41
        push.93  mem_store.42
        push.116 mem_store.43
        push.146 mem_store.44
        push.189 mem_store.45
        push.251 mem_store.46
        push.81  mem_store.47
        push.199 mem_store.48
        push.97  mem_store.49
        push.38  mem_store.50
        push.28  mem_store.51
        push.73  mem_store.52
        push.179 mem_store.53
        push.95  mem_store.54
        push.84  mem_store.55
        push.152 mem_store.56
        push.48  mem_store.57
        push.35  mem_store.58
        push.119 mem_store.59
        push.49  mem_store.60
        push.88  mem_store.61
        push.242 mem_store.62
        push.3   mem_store.63
        push.148 mem_store.64
        push.169 mem_store.65
        push.72  mem_store.66
        push.120 mem_store.67
        push.62  mem_store.68
        push.161 mem_store.69
        push.166 mem_store.70
        push.83  mem_store.71
        push.175 mem_store.72
        push.191 mem_store.73
        push.137 mem_store.74
        push.19  mem_store.75
        push.100 mem_store.76
        push.129 mem_store.77
        push.112 mem_store.78
        push.55  mem_store.79
        push.221 mem_store.80
        push.102 mem_store.81
        push.218 mem_store.82
        push.61  mem_store.83
        push.151 mem_store.84
        push.237 mem_store.85
        push.68  mem_store.86
        push.164 mem_store.87
        push.17  mem_store.88
        push.147 mem_store.89
        push.46  mem_store.90
        push.234 mem_store.91
        push.203 mem_store.92
        push.216 mem_store.93
        push.22  mem_store.94
        push.141 mem_store.95
        push.65  mem_store.96
        push.57  mem_store.97
        push.123 mem_store.98
        push.12  mem_store.99
        push.244 mem_store.100
        push.54  mem_store.101
        push.219 mem_store.102
        push.231 mem_store.103
        push.96  mem_store.104
        push.77  mem_store.105
        push.180 mem_store.106
        push.154 mem_store.107
        push.5   mem_store.108
        push.253 mem_store.109
        push.133 mem_store.110
        push.165 mem_store.111
        push.98  mem_store.112
        push.195 mem_store.113
        push.205 mem_store.114
        push.134 mem_store.115
        push.245 mem_store.116
        push.30  mem_store.117
        push.9   mem_store.118
        push.188 mem_store.119
        push.59  mem_store.120
        push.142 mem_store.121
        push.186 mem_store.122
        push.197 mem_store.123
        push.181 mem_store.124
        push.144 mem_store.125
        push.92  mem_store.126
        push.31  mem_store.127
        push.224 mem_store.128
        push.163 mem_store.129
        push.111 mem_store.130
        push.74  mem_store.131
        push.58  mem_store.132
        push.69  mem_store.133
        push.113 mem_store.134
        push.196 mem_store.135
        push.67  mem_store.136
        push.246 mem_store.137
        push.225 mem_store.138
        push.10  mem_store.139
        push.121 mem_store.140
        push.50  mem_store.141
        push.60  mem_store.142
        push.157 mem_store.143
        push.90  mem_store.144
        push.122 mem_store.145
        push.2   mem_store.146
        push.250 mem_store.147
        push.101 mem_store.148
        push.75  mem_store.149
        push.178 mem_store.150
        push.159 mem_store.151
        push.24  mem_store.152
        push.36  mem_store.153
        push.201 mem_store.154
        push.11  mem_store.155
        push.243 mem_store.156
        push.132 mem_store.157
        push.198 mem_store.158
        push.190 mem_store.159
        push.114 mem_store.160
        push.233 mem_store.161
        push.39  mem_store.162
        push.52  mem_store.163
        push.21  mem_store.164
        push.209 mem_store.165
        push.108 mem_store.166
        push.238 mem_store.167
        push.91  mem_store.168
        push.187 mem_store.169
        push.18  mem_store.170
        push.104 mem_store.171
        push.194 mem_store.172
        push.37  mem_store.173
        push.153 mem_store.174
        push.34  mem_store.175
        push.200 mem_store.176
        push.143 mem_store.177
        push.126 mem_store.178
        push.155 mem_store.179
        push.236 mem_store.180
        push.118 mem_store.181
        push.64  mem_store.182
        push.80  mem_store.183
        push.172 mem_store.184
        push.89  mem_store.185
        push.94  mem_store.186
        push.193 mem_store.187
        push.135 mem_store.188
        push.183 mem_store.189
        push.86  mem_store.190
        push.107 mem_store.191
        push.252 mem_store.192
        push.13  mem_store.193
        push.167 mem_store.194
        push.206 mem_store.195
        push.136 mem_store.196
        push.220 mem_store.197
        push.207 mem_store.198
        push.103 mem_store.199
        push.171 mem_store.200
        push.160 mem_store.201
        push.76  mem_store.202
        push.182 mem_store.203
        push.227 mem_store.204
        push.217 mem_store.205
        push.158 mem_store.206
        push.56  mem_store.207
        push.174 mem_store.208
        push.4   mem_store.209
        push.66  mem_store.210
        push.109 mem_store.211
        push.139 mem_store.212
        push.162 mem_store.213
        push.184 mem_store.214
        push.211 mem_store.215
        push.249 mem_store.216
        push.47  mem_store.217
        push.125 mem_store.218
        push.232 mem_store.219
        push.117 mem_store.220
        push.43  mem_store.221
        push.16  mem_store.222
        push.42  mem_store.223
        push.127 mem_store.224
        push.20  mem_store.225
        push.241 mem_store.226
        push.25  mem_store.227
        push.149 mem_store.228
        push.105 mem_store.229
        push.156 mem_store.230
        push.51  mem_store.231
        push.53  mem_store.232
        push.168 mem_store.233
        push.145 mem_store.234
        push.247 mem_store.235
        push.223 mem_store.236
        push.79  mem_store.237
        push.78  mem_store.238
        push.226 mem_store.239
        push.15  mem_store.240
        push.222 mem_store.241
        push.82  mem_store.242
        push.115 mem_store.243
        push.70  mem_store.244
        push.210 mem_store.245
        push.27  mem_store.246
        push.41  mem_store.247
        push.1   mem_store.248
        push.170 mem_store.249
        push.40  mem_store.250
        push.131 mem_store.251
        push.192 mem_store.252
        push.229 mem_store.253
        push.248 mem_store.254
        push.255 mem_store.255
    end

    proc.tip5_sbox_layer
        # Since the Tip5 initialization procedure has dumped the lookup table into addresses
        # 0..255, we can simply use the memory load instruction to do the lookup.
        mem_load
        swap.1  mem_load swap.1
        swap.2  mem_load swap.2
        swap.3  mem_load swap.3
        swap.4  exp.7    swap.4
        swap.5  exp.7    swap.5
        swap.6  exp.7    swap.6
        swap.7  exp.7    swap.7
        swap.8  exp.7    swap.8
        swap.9  exp.7    swap.9
        swap.10 exp.7    swap.10
        swap.11 exp.7    swap.11
        swap.12 exp.7    swap.12
        swap.13 exp.7    swap.13
        swap.14 exp.7    swap.14
        swap.15 exp.7    swap.15
    end

    proc.tip5_round_0
        exec.tip5_sbox_layer
    end

    proc.tip5_round_1
        exec.tip5_sbox_layer
    end

    proc.tip5_round_2
        exec.tip5_sbox_layer
    end

    proc.tip5_round_3
        exec.tip5_sbox_layer
    end

    proc.tip5_round_4
        exec.tip5_sbox_layer
    end

    proc.tip5
       exec.tip5_round_0
       exec.tip5_round_1
       exec.tip5_round_2
       exec.tip5_round_3
       exec.tip5_round_4
    end

    begin
        call.tip5_init
        call.tip5
    end
";

fn main() {
    let assembler = Assembler::default()
        .with_library(&StdLibrary::default())
        .unwrap();

    let program = assembler.compile(TIP5_LIB).unwrap();
    let stack_input =
        StackInputs::try_from_values([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
            .unwrap();

    let (outputs, proof) = prove(
        &program,
        stack_input,
        MemAdviceProvider::default(),
        ProofOptions::default(),
    )
    .unwrap();

    let program_info = ProgramInfo::new(program.hash(), Kernel::default());
    match verify(program_info, StackInputs::default(), outputs, proof) {
        Ok(_) => println!("Execution verified!"),
        Err(msg) => println!("Something went terribly wrong: {msg}"),
    }
}

#[cfg(test)]
mod tests {
    use miden_vm::execute;
    use miden_vm::MemAdviceProvider;
    use miden_vm::StackInputs;

    use crate::*;

    #[test]
    fn compliance() {
        let assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .unwrap();

        let program = assembler.compile(TIP5_LIB).unwrap();

        let stack_inputs =
            StackInputs::try_from_values([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
                .unwrap();
        let advice_provider = MemAdviceProvider::default();
        let trace = execute(&program, stack_inputs, advice_provider).unwrap();
        let public_output = trace.stack_outputs().stack();

        // todo: fix this test
        println!("public output: {public_output:?}");
        // let expected_output = vec![0; 16];
        // assert_eq!(expected_output, public_output);
    }
}
